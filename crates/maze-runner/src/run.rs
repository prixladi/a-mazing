use std::collections::VecDeque;

use maze_core::{Position, TileBoard};

use super::nodes::Nodes;

pub(super) fn run_maze(
    board: &TileBoard,
    ascending_checkpoint_levels: &Vec<i32>,
    entrypoint_position: &Position,
) -> Option<MazeRunResult> {
    let mut nodes = Nodes::new(board);
    let entrypoint_node = nodes.get_node_mut(entrypoint_position);
    entrypoint_node.set_distance(ascending_checkpoint_levels[0], 0);

    let mut queue = VecDeque::new();
    queue.push_back((*entrypoint_position, 0, 0));

    let (exit_position, distance) = loop {
        // if the queue is empty we can safely say that there is no solution to this maze
        let (current_position, current_distance, current_level_index) = queue.pop_front()?;

        // this means we entered checkpoint of the last layer and the maze is solved
        if current_level_index >= ascending_checkpoint_levels.len() {
            break (current_position, current_distance);
        }

        let current_level = ascending_checkpoint_levels[current_level_index];

        get_eligible_neighbors(&nodes, &current_position, current_level)
            .into_iter()
            .for_each(|neighbor_pos| {
                let neighbor = nodes.get_node_mut(&neighbor_pos);
                let mut neighbor_level_index = current_level_index;
                let neighbor_distance = current_distance + 1;

                if neighbor.is_checkpoint(current_level) {
                    neighbor_level_index += 1;

                    // if we progress to the next level we need to set checkpoint distance for next level as a entrypoint for that level
                    if neighbor_level_index < ascending_checkpoint_levels.len() {
                        let next_level = ascending_checkpoint_levels[neighbor_level_index];
                        neighbor.set_distance_if_not_set(next_level, neighbor_distance);
                    }
                }

                neighbor.set_distance(current_level, neighbor_distance);
                queue.push_back((neighbor_pos, neighbor_distance, neighbor_level_index));
            })
    };

    Some(MazeRunResult {
        asc_checkpoint_levels: ascending_checkpoint_levels.clone(),
        exit_position,
        evaluated_nodes: nodes,
        distance,
    })
}

fn get_eligible_neighbors(
    nodes: &Nodes,
    current_position: &Position,
    current_level: i32,
) -> Vec<Position> {
    nodes
        .get_neighbors_positions(&current_position)
        .iter()
        .cloned()
        .filter(|neighbor_pos| {
            let neighbor = nodes.get_node(&neighbor_pos);
            neighbor.can_enter() && !neighbor.has_distance(current_level)
        })
        .collect()
}

pub struct MazeRunResult {
    asc_checkpoint_levels: Vec<i32>,
    exit_position: Position,
    evaluated_nodes: Nodes,
    distance: u32,
}

impl MazeRunResult {
    pub fn solved_path(&self) -> Vec<Position> {
        let mut best_path = Vec::with_capacity(self.distance as usize + 1);
        best_path.push(self.evaluated_nodes.get_node(&self.exit_position));

        let mut remaining_levels = self.asc_checkpoint_levels.clone();

        let mut previous_level = remaining_levels.pop();
        let mut current_level = remaining_levels.pop();

        let mut iterations_remaining = self.distance + 2;
        loop {
            iterations_remaining -= 1;
            if iterations_remaining <= 0 {
                panic!("Unable to get solved path for solved maze run! Too many iterations.");
            }

            let current_node = best_path[best_path.len() - 1];

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
                .get_lowest_distance_neighbor(&current_node.position(), previous_level.unwrap())
                .expect("Expected to find lowest distance neighbor for solved maze run!");

            best_path.push(neighbor);
        }

        best_path
            .iter()
            .rev()
            .map(|node| node.position())
            .copied()
            .collect()
    }

    pub fn score(&self) -> u32 {
        // currently score equates to distance but in the future it could deviate with modifiers usage
        self.distance
    }
}
