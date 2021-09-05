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
    inland: bool,
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
            {
                let mut node = &mut graph[tn_idx];
                node.local_trade_value = gs_node.local_value;
                node.collecting_trade_power = gs_node.collector_power;
                node.transfer_trade_power = gs_node.pull_power;
                node.inland = data.trade[&gs_node.definitions].inland;
            }
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

    fn adjust_trade(&mut self, node_idx: NodeIndex, trade_power: FixedPoint) {
        self.graph[node_idx].our_trade_power += trade_power;
        if self.is_collecting(node_idx) {
            self.graph[node_idx].collecting_trade_power += trade_power;
        } else {
            self.graph[node_idx].transfer_trade_power += trade_power;
            if let Some(dir) = self.get_steer_direction(node_idx) {
                let steer_modifier = self.graph[node_idx].our_steer_modifier;
                let edge_idx = self.graph.find_edge(node_idx, dir).unwrap();
                self.graph[edge_idx].trade_power_pushing +=
                    trade_power * steer_modifier;
            }
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

    fn compute_trade(&self, trade_values: &mut [FixedPoint],
                     trade_fractions: &mut [f64],
                     trade_derivatives: &mut [f64]) {
        self.compute_trade_values(trade_values);
        self.compute_trade_fractions(trade_fractions);
        for node_idx in self.graph.node_indices() {
            trade_derivatives[node_idx.index()] =
                self.derivative(node_idx, trade_values, trade_fractions);
        }
    }

    fn print_best_nodes(&self, trade_values: &[FixedPoint],
                        trade_fractions: &[f64], trade_derivatives: &[f64]) {
        let mut node_indices : Vec<_> = self.graph.node_indices()
            .map(NodeIndex::index).collect();
        node_indices.sort_unstable_by(|&b, &a| {
            trade_derivatives[a].partial_cmp(&trade_derivatives[b]).unwrap()
        });
        for i in node_indices {
            if trade_derivatives[i] < 0.01 { break; }
            println!("{}: {:.6} (TV: {}, TF: {:.6})", self.names[i],
                     trade_derivatives[i], trade_values[i], trade_fractions[i]);
        }
    }
}

pub fn optimize_trade(data: &GameData, gamestate: &Gamestate,
                      country: &Eu4Atom) {
    let mut tn = TradeNetwork::new(data, gamestate, country);
    //tn.display_dot();
    let num_nodes = tn.graph.node_count();
    let mut trade_values = vec![Default::default(); num_nodes];
    let mut trade_fractions = vec![Default::default(); num_nodes];
    let mut trade_derivatives = vec![Default::default(); num_nodes];


    println!("Current trade derivatives:");
    tn.compute_trade(&mut trade_values, &mut trade_fractions,
                     &mut trade_derivatives);
    tn.print_best_nodes(&trade_values, &trade_fractions, &trade_derivatives);

    let modifiers = gamestate.countries[country]
        .get_modifiers(data, gamestate, country);
    let mut num_ships = 0;
    let mut ship_power = FixedPoint::ZERO;
    for gs_node in &gamestate.trade.node {
        let name = &gs_node.definitions;
        let tn_idx = 
            NodeIndex::new(tn.names.iter().position(|k| k == name).unwrap());
        for (tag, country_trade) in &gs_node.country_info {
            if tag == country {
                num_ships += country_trade.light_ship;
                ship_power += country_trade.ship_power;
                tn.adjust_trade(tn_idx, -country_trade.ship_power);
            }
        }
    }
    ship_power = ship_power / FixedPoint::from(num_ships);
    println!("{} ships of average {} power each", num_ships, ship_power);
    println!("global_ship_power: {}",
             modifiers[&eu4_atom!("global_ship_trade_power")]
                .as_fixed_point());

    println!("Sans ships:");
    tn.compute_trade(&mut trade_values, &mut trade_fractions,
                     &mut trade_derivatives);
    tn.print_best_nodes(&trade_values, &trade_fractions, &trade_derivatives);

    let mut ship_allocation = vec![0; num_nodes];
    for _ in 0..num_ships {
        let best = tn.graph.node_indices()
            .filter(|&n| !tn.graph[n].inland)
            .max_by(|&a, &b| trade_derivatives[a.index()]
                    .partial_cmp(&trade_derivatives[b.index()]).unwrap())
            .unwrap();
        ship_allocation[best.index()] += 1;
        tn.adjust_trade(best, ship_power);
        tn.compute_trade(&mut trade_values, &mut trade_fractions,
                         &mut trade_derivatives);
    }

    println!("Ship allocation:");
    for node_idx in tn.graph.node_indices() {
        let ship_count = ship_allocation[node_idx.index()];
        if ship_count > 0 {
            println!("Add {} ships to {}", ship_count,
                     tn.names[node_idx.index()]);
        }
    }

    println!("Post allocation:");
    tn.print_best_nodes(&trade_values, &trade_fractions, &trade_derivatives);
}
