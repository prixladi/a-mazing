use std::collections::VecDeque;

use crate::Position;

use super::node::Nodes;

pub struct Run {
    distance: u32,
    exit_position: Position,
    evaluated_nodes: Nodes,
}

impl Run {
    pub fn new(mut nodes: Nodes, entrance_position: Position) -> Option<Run> {
        let mut queue = VecDeque::new();
        queue.push_back((entrance_position, 0));

        let (exit_position, distance) = loop {
            let (position, distance) = queue.pop_front()?;

            let node = nodes.get_node_mut(&position);
            node.set_distance(distance);
            if node.is_exit() {
                break (node.get_position(), distance);
            }

            for neighbor in nodes.get_neighbors(&position) {
                if neighbor.can_enter() && !neighbor.has_distance() {
                    let position = neighbor.get_position();
                    queue.push_back((position, distance + 1));
                }
            }
        };

        Some(Self {
            distance,
            exit_position,
            evaluated_nodes: nodes,
        })
    }

    pub fn get_solved_path(&self) -> Vec<Position> {
        let mut best_path = Vec::with_capacity(self.distance as usize + 1);
        best_path.push(self.evaluated_nodes.get_node(&self.exit_position));

        let mut iterations_remaining = self.get_distance() + 2;
        loop {
            iterations_remaining -= 1;
            if iterations_remaining <= 0 {
                panic!("Unable to get solved path for solved board run!");
            }

            let current_node = best_path.last().unwrap();
            if current_node.is_entrance() {
                break;
            }

            let neighbor = self
                .evaluated_nodes
                .get_lowest_distance_neighbor(&current_node.get_position())
                .expect("Expected to find lowest distance neighbor for solved solved board run!");

            best_path.push(neighbor);
        }

        best_path
            .iter()
            .rev()
            .map(|node| node.get_position())
            .collect()
    }

    pub fn get_distance(&self) -> u32 {
        self.distance
    }
}

