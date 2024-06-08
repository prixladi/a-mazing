use std::collections::VecDeque;

use crate::{core::tile::TileKind, Position};

use super::nodes::Nodes;

pub struct Run {
    asc_checkpoint_levels: Vec<i32>,
    exit_position: Position,
    evaluated_nodes: Nodes,
    distance: u32,
}

impl Run {
    pub fn execute(
        tiles: &Vec<Vec<TileKind>>,
        asc_checkpoint_levels: &Vec<i32>,
        entrypoint_position: Position,
    ) -> Option<Run> {
        let mut nodes = Nodes::new(tiles);
        let entrypoint_node = nodes.get_node_mut(&entrypoint_position);
        entrypoint_node.set_distance(asc_checkpoint_levels[0], 0);

        let mut queue = VecDeque::new();
        queue.push_back((entrypoint_position, 0, 0));

        let (exit_position, distance) = loop {
            // if the queue is empty we can safely say that there is no solution to this maze
            let (current_position, current_distance, current_level_index) = queue.pop_front()?;

            // this means we entered checkpoint of the last layer and the maze is solved
            if current_level_index >= asc_checkpoint_levels.len() {
                break (current_position, current_distance);
            }

            let current_level = asc_checkpoint_levels[current_level_index];
            nodes
                .get_neighbors_positions(&current_position)
                .iter()
                .cloned()
                .for_each(|neighbor_pos| {
                    let neighbor = nodes.get_node_mut(&neighbor_pos);
                    if !neighbor.can_enter() || neighbor.has_distance(current_level) {
                        return;
                    }

                    let mut neighbor_level_index = current_level_index;
                    let neighbor_distance = current_distance + 1;

                    if neighbor.is_checkpoint(current_level) {
                        neighbor_level_index += 1;

                        // if we progress to the next level we need to set checkpoint distance for next level as a entrypoint
                        if neighbor_level_index < asc_checkpoint_levels.len() {
                            let next_level = asc_checkpoint_levels[neighbor_level_index];
                            neighbor.set_distance_if_not_set(next_level, neighbor_distance);
                        }
                    }

                    neighbor.set_distance(current_level, neighbor_distance);
                    queue.push_back((neighbor_pos, neighbor_distance, neighbor_level_index));
                })
        };

        Some(Self {
            asc_checkpoint_levels: asc_checkpoint_levels.clone(),
            exit_position,
            evaluated_nodes: nodes,
            distance,
        })
    }

    pub fn get_solved_path(&self) -> Vec<Position> {
        let mut best_path = Vec::with_capacity(self.distance as usize + 1);
        best_path.push(self.evaluated_nodes.get_node(&self.exit_position));

        let mut remaining_levels = self.asc_checkpoint_levels.clone();

        let mut previous_level = remaining_levels.pop();
        let mut current_level = remaining_levels.pop();

        let mut iterations_remaining = self.get_distance() + 2;
        loop {
            iterations_remaining -= 1;
            if iterations_remaining <= 0 {
                panic!("Unable to get solved path for solved maze run! Too many iterations.");
            }

            let current_node = best_path.last().unwrap();

            match current_level {
                Some(level) if current_node.is_checkpoint(level) => {
                    previous_level = current_level;
                    current_level = remaining_levels.pop()
                }
                None if current_node.is_entrypoint() => break,
                _ => (),
            }

            let neighbor = self
                .evaluated_nodes
                .get_lowest_distance_neighbor(&current_node.get_position(), previous_level.unwrap())
                .expect("Expected to find lowest distance neighbor for solved maze run!");

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
