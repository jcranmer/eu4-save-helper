use eu4::{GameData, Gamestate};
use paradox::FixedPoint;
use petgraph::graph::{EdgeIndex, Graph, NodeIndex};
use petgraph::visit::{EdgeRef, Topo, Walker};
use petgraph::Direction::Outgoing;

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
}

impl SimpleTradeNode {
    fn total_trade_power(&self) -> FixedPoint {
        self.collecting_trade_power + self.transfer_trade_power
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
    names: Vec<String>,
    postorder: Vec<NodeIndex>,
    trade_efficiency: FixedPoint,
    collecting: Vec<NodeIndex>,
    steering: Vec<EdgeIndex>,
}

impl TradeNetwork {
    fn new(data: &GameData, gamestate: &Gamestate, us: &str) -> Self {
        // Build the graph structure from the game data.
        let trade_box = data.base_info.get_id_box::<eu4::TradeNode>();
        let mut names = Vec::with_capacity(trade_box.len());
        let mut graph = SimpleTradeGraph::with_capacity(trade_box.len(),
            trade_box.len() * 4);
        for i in 1..=trade_box.len() {
            graph.add_node(Default::default());
            names.push(trade_box.get_string(i as u32).into());
        }
        for (key, value) in &data.trade {
            let source = NodeIndex::new(key.index as usize - 1);
            // Reverse the edges. Petgraph seems to put the newest edge at the
            // beginning of the list, but we want it to be at the end for
            // later usage in indexing.
            for edge in value.outgoing.iter().rev() {
                let target_idx = trade_box.get_index(&edge.name).unwrap();
                let target = NodeIndex::new(target_idx as usize - 1);
                graph.add_edge(source, target, Default::default());
            }
        }

        // Initialize node information from gamestate.
        let mut merchant_collects = Vec::with_capacity(4);
        let mut merchant_steers = Vec::with_capacity(16);
        for gs_node in &gamestate.trade.node {
            let tn_idx = NodeIndex::new(gs_node.definitions.index as usize - 1);
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
            for (tag, country_trade) in &gs_node.country_info {
                if country_trade.val == Default::default() { continue; }
                let merchant_type = country_trade.r#type;
                let steering = country_trade.has_trader &&
                    merchant_type == 1;
                let collecting = country_trade.has_capital ||
                    (country_trade.has_trader && merchant_type == 0);

                if steering {
                    let steer_direction = country_trade.steer_power as usize;
                    let edge_idx = graph.edges(tn_idx)
                        .nth(steer_direction)
                        .unwrap().id();
                    // XXX: Weight by trade steering? How do I find this?
                    graph[edge_idx].trade_power_pushing =
                        country_trade.val;
                    if tag == us {
                        merchant_steers.push(edge_idx);
                    }
                }

                if tag == us {
                    if collecting {
                        merchant_collects.push(tn_idx);
                    }
                    graph[tn_idx].our_trade_power = country_trade.val;
                }
                println!("  {} power: {}", tag, country_trade.val);
            }

            // XXX this gets us past the trade steering issue.
            let edge_idx : Vec<_> = graph.edges(tn_idx)
                .map(|e| e.id())
                .collect();
            for (&e, &i) in edge_idx.iter().zip(gs_node.steer_power.iter()) {
                graph[e].trade_power_pushing = i;
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
            trade_efficiency: Default::default(),
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
        self.graph.edges_directed(node_idx, Outgoing)
            .map(|e| e.weight().trade_power_pushing)
            .sum()
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
                    f64::from(node.total_trade_power());
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
        let node = &self.graph[node_idx];
        let we_steer = self.get_steer_direction(node_idx);

        let push_fraction = f64::from(node.push_fraction());
        let tot_squared = f64::from(node.total_trade_power() * node.total_trade_power());
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
            }) / f64::from(all_steering * all_steering);

            (
                push_fraction * steer_derivative +
                steer_fraction * push_derivative
            ) * f64::from(e.weight().steering_bonus) *
                trade_fractions[e.target().index()]
        }).sum::<f64>() + if self.is_collecting(node_idx) {
            -push_derivative
        } else {
            Default::default()
        }) * f64::from(trade_values[node_idx.index()])
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
    let tn = TradeNetwork::new(data, gamestate, country);
    let num_nodes = tn.graph.node_count();
    let mut trade_values = vec![Default::default(); num_nodes];
    let mut trade_fractions = vec![Default::default(); num_nodes];
    tn.compute_trade_values(&mut trade_values);
    tn.compute_trade_fractions(&mut trade_fractions);
    for node_idx in tn.graph.node_indices() {
        let i = node_idx.index();
        println!("{}: {}, {:.6}", tn.names[i], trade_values[i], trade_fractions[i]);
        println!("Derivative: {:.6}", tn.derivative(node_idx, &trade_values, &trade_fractions));
    }
    for idx in tn.collecting {
        println!("Collecting in {}", tn.names[idx.index()]);
    }
    for e in tn.graph.edge_references() {
        if tn.steering.contains(&e.id()) {
            println!("Steering {}->{}", tn.names[e.source().index()], tn.names[e.target().index()]);
        }
    }
    //tn.display_dot();
}
