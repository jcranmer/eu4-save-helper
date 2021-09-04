use eu4::{eu4_atom, Eu4Atom, GameData, Gamestate};
use paradox::FixedPoint;
use petgraph::graph::{EdgeIndex, Graph, NodeIndex};
use petgraph::visit::{EdgeRef, Topo, Walker};
use petgraph::Direction::Outgoing;
use std::collections::HashMap;

// XXX: move this elsewhere
struct ModifierCache<'a> {
    gamedata: &'a GameData,
    gamestate: &'a Gamestate,
    cache: std::cell::RefCell<HashMap<Eu4Atom, eu4::Modifiers>>
}

impl <'a> ModifierCache<'a> {
    pub fn new(gamedata: &'a GameData, gamestate: &'a Gamestate) -> Self {
        Self {
            gamedata, gamestate,
            cache: std::cell::RefCell::new(Default::default())
        }
    }

    pub fn get_modifier(&self, country: &Eu4Atom,
                        modifier: &Eu4Atom) -> FixedPoint {
        self.cache.borrow_mut()
            .entry(country.clone())
            .or_insert_with(|| {
                self.gamestate.countries[country]
                    .get_modifiers(self.gamedata, self.gamestate, &country)
            })[modifier].as_fixed_point()
    }
}

// Trade steering notes:
// This isn't saved in the save file anywhere, it looks like, has to be
// recomputed from scratch. (Nor is it readily available in UI, for that
// matter).
// Steering power = trade power * trade steering modifier
// Steering bonus = trade steering modifier * 0.05 / n, until n > 5.
// Country order for steering bonus may be tag order?

#[derive(Default)]
struct SimpleTradeNode {
    local_trade_value: FixedPoint,
    collecting_trade_power: FixedPoint,
    transfer_trade_power: FixedPoint,
    our_trade_power: FixedPoint,
    our_steer_modifier: FixedPoint,
    has_steering: bool,
    trade_efficiency: FixedPoint,
    trade_power_modifier: FixedPoint,
}

impl SimpleTradeNode {
    fn total_trade_power(&self) -> FixedPoint {
        let val = self.collecting_trade_power + self.transfer_trade_power;
        if val == FixedPoint::ZERO {
            FixedPoint::ONE
        } else {
            val
        }
    }

    fn push_fraction(&self) -> FixedPoint {
        self.transfer_trade_power / self.total_trade_power()
    }
}

#[derive(Default)]
struct SimpleTradeEdge {
    trade_power_pushing: FixedPoint,
    steering_bonus: FixedPoint
}

type SimpleTradeGraph = Graph<SimpleTradeNode, SimpleTradeEdge>;

struct TradeNetwork {
    graph: SimpleTradeGraph,
    names: Vec<Eu4Atom>,
    postorder: Vec<NodeIndex>,
    collecting: Vec<NodeIndex>,
    steering: Vec<EdgeIndex>,
}

impl TradeNetwork {
    fn new(data: &GameData, gamestate: &Gamestate, us: &Eu4Atom) -> Self {
        // Build the graph structure from the game data.
        let names : Vec<_> = data.trade.get_names()
            .map(|name| name.clone())
            .collect();
        let mut graph = SimpleTradeGraph::with_capacity(names.len(),
            names.len() * 4);
        for _ in &names {
            graph.add_node(Default::default());
        }
        let get_index = |name: &Eu4Atom| {
            NodeIndex::new(names.iter().position(|k| k == name).unwrap())
        };
        for name in &names {
            let source = get_index(name);
            let value = &data.trade[name];
            // Reverse the edges. Petgraph seems to put the newest edge at the
            // beginning of the list, but we want it to be at the end for
            // later usage in indexing.
            for edge in value.outgoing.iter().rev() {
                let target = get_index(&edge.name);
                graph.add_edge(source, target, Default::default());
            }
        }

        // Cache for getting modifier information from a country.
        let modifier_cache = ModifierCache::new(data, gamestate);
        let get_trade_steering = |country: &Eu4Atom| {
            modifier_cache.get_modifier(country, &eu4_atom!("trade_steering"))
                + FixedPoint::ONE
        };
        //let global_trade_power =
        //    modifier_cache.get_modifier(us, &eu4_atom!("global_trade_power"));
        //let domestic_trade_power = modifier_cache.get_modifier(us,
        //    &eu4_atom!("global_own_trade_power"));
        //let foreign_trade_power = modifier_cache.get_modifier(us,
        //    &eu4_atom!("global_foreign_trade_power")) -
        //    gamestate.countries[us].overextension_percentage;

        // Initialize node information from gamestate.
        let mut merchant_collects = Vec::with_capacity(4);
        let mut merchant_steers = Vec::with_capacity(16);
        for gs_node in &gamestate.trade.node {
            let tn_idx = get_index(&gs_node.definitions);
            println!("{}:", names[tn_idx.index()]);
            {
                let mut node = &mut graph[tn_idx];
                node.local_trade_value = gs_node.local_value;
                node.collecting_trade_power = gs_node.collector_power;
                node.transfer_trade_power = gs_node.pull_power;
            }
            println!("  current: {}", gs_node.current);
            println!("  local_value: {}", gs_node.local_value);
            println!("  outgoing: {}", gs_node.outgoing);
            println!("  value_added_outgoing: {}", gs_node.value_added_outgoing);
            println!("  steer_power: {:?}", gs_node.steer_power);
            println!("  collector_power: {}", gs_node.collector_power);
            println!("  pull_power: {}", gs_node.pull_power);
            let mut total_trade_steer = FixedPoint::ZERO;
            //let max_p_power = gs_node.country_info
            //    .iter().map(|(_, x)| x.province_power).max()
            //    .unwrap_or_default();
            for (tag, country_trade) in &gs_node.country_info {
                let trade_power = country_trade.val + country_trade.t_in -
                    country_trade.t_out;
                if trade_power == FixedPoint::ZERO { continue; }
                let merchant_type = country_trade.r#type;
                let steering = country_trade.has_trader &&
                    merchant_type == 1;
                let collecting = country_trade.has_capital ||
                    (country_trade.has_trader && merchant_type == 0);
                //let is_domestic = country_trade.has_capital ||
                //    country_trade.province_power == max_p_power;

                // Calculate steering power
                if steering {
                    let steer_direction = country_trade.steer_power as usize;
                    let edge_idx = graph.edges(tn_idx)
                        .nth(steer_direction)
                        .unwrap().id();
                    let steer_modifier = get_trade_steering(tag);
                    let trade_steer = trade_power * steer_modifier;
                    graph[edge_idx].trade_power_pushing += trade_steer;
                    total_trade_steer += trade_steer;
                    if tag == us {
                        merchant_steers.push(edge_idx);
                        graph[tn_idx].our_steer_modifier = steer_modifier;
                    }
                }

                if tag == us {
                    if collecting {
                        merchant_collects.push(tn_idx);
                        // Estimate trade efficiency in this node instead of
                        // calculating it from first principles.
                        graph[tn_idx].trade_efficiency =
                            country_trade.money / country_trade.total;
                    }
                    graph[tn_idx].our_trade_power = trade_power;
                    graph[tn_idx].trade_power_modifier =
                        country_trade.max_demand;
                    //if !country_trade.trading_policy.as_ref().is_empty() {
                    //    graph[tn_idx].trade_power_modifier +=
                    //        data.trade_policy[&country_trade.trading_policy]
                    //            .get_trade_power_modifier();
                    //}
                }
            }

            let has_steering = total_trade_steer != FixedPoint::ZERO;
            graph[tn_idx].has_steering = has_steering;

            // Get the game's trade steering fraction. We'll only use it when
            // there's nobody steering, but make sure our notes are correct in
            // case there's a trade steering bonus we've missed somewhere.
            let edge_idx : Vec<_> = graph.edges(tn_idx)
                .map(|e| e.id())
                .collect();
            for (&e, &i) in edge_idx.iter().zip(gs_node.steer_power.iter()) {
                if !has_steering {
                    graph[e].trade_power_pushing = i;
                } else {
                    let calculated =
                        graph[e].trade_power_pushing / total_trade_steer;
                    if calculated != i {
                        println!("Calculated {}, actual {}", calculated, i);
                    }
                };
            }

            // Compute total trade steering bonus from the incoming links
            let mut total_value = gs_node.local_value;
            for incoming in &gs_node.incoming {
                let in_idx = NodeIndex::new(incoming.from as usize - 1);
                let e = graph.find_edge(in_idx, tn_idx)
                    .expect("Unexpected edge in the trade graph");
                let in_value = incoming.value;
                graph[e].steering_bonus = if in_value == FixedPoint::ZERO {
                    FixedPoint::ONE
                } else {
                    in_value / (in_value - incoming.add)
                };
                total_value += in_value;
            }
        }

        let traversal = Topo::new(&graph);
        let mut postorder : Vec<_> = traversal.iter(&graph).collect();
        postorder.reverse();
        Self {
            graph, names,
            postorder,
            collecting: merchant_collects,
            steering: merchant_steers,
        }
    }

    fn is_collecting(&self, node_idx: NodeIndex) -> bool {
        self.collecting.contains(&node_idx)
    }

    fn get_steer_direction(&self, node_idx: NodeIndex) -> Option<NodeIndex> {
        for target in self.graph.edges_directed(node_idx, Outgoing) {
            if self.steering.contains(&target.id()) {
                return Some(target.target());
            }
        }
        None
    }

    fn steering_power(&self, node_idx: NodeIndex) -> FixedPoint {
        let val = self.graph.edges_directed(node_idx, Outgoing)
            .map(|e| e.weight().trade_power_pushing)
            .sum();
        // Adjust the total sum by 1 if it's 0 to avoid 0 / 0 calculations.
        if val == FixedPoint::ZERO {
            FixedPoint::ONE
        } else {
            val
        }
    }

    fn compute_trade_values(&self, value: &mut [FixedPoint]) {
        value.fill(Default::default());
        for &node_idx in self.postorder.iter().rev() {
            let node = &self.graph[node_idx];
            value[node_idx.index()] += node.local_trade_value;
            let total_value = value[node_idx.index()];
            let transfer_value = total_value * node.push_fraction();
            let all_steering = self.steering_power(node_idx);
            for edge in self.graph.edges_directed(node_idx, Outgoing) {
                let target = edge.target();
                let edge = edge.weight();
                value[target.index()] +=
                    transfer_value * edge.trade_power_pushing / all_steering *
                    edge.steering_bonus;
            }
        }
    }

    fn compute_trade_fractions(&self, fraction: &mut [f64]) {
        // zero out fractions first.
        fraction.fill(Default::default());
        for &node_idx in &self.postorder {
            let node = &self.graph[node_idx];
            if self.is_collecting(node_idx) {
                fraction[node_idx.index()] += f64::from(node.our_trade_power) /
                    f64::from(node.total_trade_power()) *
                    f64::from(node.trade_efficiency);
            }
            let push_fraction = f64::from(node.push_fraction());
            let all_steering = self.steering_power(node_idx);
            for edge in self.graph.edges_directed(node_idx, Outgoing) {
                let target = edge.target();
                let edge = edge.weight();
                fraction[node_idx.index()] +=
                    push_fraction *
                    f64::from(edge.trade_power_pushing / all_steering) *
                    f64::from(edge.steering_bonus) * fraction[target.index()];
            }
        }
    }

    fn derivative(&self, node_idx: NodeIndex, trade_values: &[FixedPoint],
                  trade_fractions: &[f64]) -> f64 {
        // Notes for how we take the derivative of trade value wrt a trade node:
        // TV_i = TV_i * (collect fraction + PF_i * Sum(SF_j * TF_j))
        // TV_i' = TV_i * (collect fraction' + PF_i' * Sum(SF_j * TF_j)
        //                                   + PF_i * Sum(SF_j' * TF_j))
        let node = &self.graph[node_idx];
        let we_steer = self.get_steer_direction(node_idx);

        let push_fraction = f64::from(node.push_fraction());
        let tot_squared = f64::from(node.total_trade_power()) *
            f64::from(node.total_trade_power());
        let push_derivative = if self.is_collecting(node_idx) {
            f64::from(-node.transfer_trade_power) / tot_squared
        } else {
            f64::from(node.collecting_trade_power) / tot_squared
        };

        let all_steering = self.steering_power(node_idx);
        (self.graph.edges_directed(node_idx, Outgoing).map(|e| {
            let steer_fraction = f64::from(e.weight().trade_power_pushing / all_steering);
            let steer_derivative = f64::from(if we_steer == Some(e.target()) {
                // Merchant steers this edge.
                all_steering - e.weight().trade_power_pushing
            } else if we_steer.is_some() {
                // Merchant steers, but not this edge.
                -e.weight().trade_power_pushing
            } else {
                // No merchant steering.
                Default::default()
            }) * f64::from(self.graph[node_idx].our_steer_modifier)
               / f64::from(all_steering * all_steering);

            (
                push_fraction * steer_derivative +
                steer_fraction * push_derivative
            ) * f64::from(e.weight().steering_bonus) *
                trade_fractions[e.target().index()]
        }).sum::<f64>() + if self.is_collecting(node_idx) {
            f64::from(node.total_trade_power() - node.our_trade_power)
                / tot_squared
                * f64::from(node.trade_efficiency)
        } else {
            Default::default()
        }) * f64::from(trade_values[node_idx.index()])
           * f64::from(node.trade_power_modifier)
    }

    fn display_dot(&self) {
        let disp_graph = self.graph.map(
            |idx, weight| {
                format!("{}\nTV (local): {}",
                        &self.names[idx.index()],
                        weight.local_trade_value
                        )
            },
            |_, _| "");
        use petgraph::dot::Dot;
        use std::process::Command;
        let mut child = Command::new("sh")
            .arg("-c")
            .arg("dot -Tpng | display")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to launch");
        use std::io::Write;
        write!(child.stdin.take().unwrap(), "{}", Dot::new(&disp_graph))
            .unwrap();
        child.wait().expect("Failed to wait");
    }
}

pub fn optimize_trade(data: &GameData, gamestate: &Gamestate, country: &str) {
    let tn = TradeNetwork::new(data, gamestate, &Eu4Atom::from(country));
    //tn.display_dot();
    let num_nodes = tn.graph.node_count();
    let mut trade_values = vec![Default::default(); num_nodes];
    let mut trade_fractions = vec![Default::default(); num_nodes];
    tn.compute_trade_values(&mut trade_values);
    tn.compute_trade_fractions(&mut trade_fractions);
    let mut trade_derivatives = vec![Default::default(); num_nodes];
    for node_idx in tn.graph.node_indices() {
        trade_derivatives[node_idx.index()] =
            tn.derivative(node_idx, &trade_values, &trade_fractions);
    }
    let mut node_indices : Vec<_> = tn.graph.node_indices()
        .map(NodeIndex::index).collect();
    node_indices.sort_unstable_by(|&a, &b| {
        trade_derivatives[a].partial_cmp(&trade_derivatives[b]).unwrap()
    });
    for i in node_indices {
        println!("{}: {:.6} (TV: {}, TF: {:.6})", tn.names[i],
                 trade_derivatives[i], trade_values[i], trade_fractions[i]);
    }
}
