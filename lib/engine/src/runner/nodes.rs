use crate::{core::tile::TileKind, Position};

use super::node::Node;

#[derive(Debug, Clone)]
pub struct Nodes {
    data: Vec<Vec<Node>>,
}

impl Nodes {
    pub fn new(tiles: &Vec<Vec<TileKind>>) -> Self {
        let nodes = tiles
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(|(y, kind)| Node::new(*kind, (x, y)))
                    .collect()
            })
            .collect();

        Self { data: nodes }
    }

    pub fn get_node(&self, (x, y): &Position) -> &Node {
        &self.data[*x][*y]
    }

    pub fn get_node_mut(&mut self, (x, y): &Position) -> &mut Node {
        &mut self.data[*x][*y]
    }

    pub fn get_neighbors_positions(&self, (x, y): &Position) -> Vec<Position> {
        let mut neighbors = Vec::with_capacity(4);

        if y + 1 < self.data[0].len() {
            neighbors.push((*x, y + 1));
        }
        if *y > 0 {
            neighbors.push((*x, y - 1));
        }

        if x + 1 < self.data.len() {
            neighbors.push((x + 1, *y));
        }
        if *x > 0 {
            neighbors.push((x - 1, *y));
        }

        return neighbors;
    }

    pub fn get_lowest_distance_neighbor(
        &self,
        position: &Position,
        checkpoint_level: i32,
    ) -> Option<&Node> {
        let mut lowest: Option<&Node> = None;

        for neighbor_position in self.get_neighbors_positions(position) {
            let neighbor = self.get_node(&neighbor_position);

            if let Some(neighbor_distance) = neighbor.get_distance(checkpoint_level) {
                let lowest_dist = lowest.and_then(|node| node.get_distance(checkpoint_level));
                lowest = match lowest_dist {
                    Some(lowest_distance) if lowest_distance <= neighbor_distance => lowest,
                    _ => Some(neighbor),
                }
            }
        }

        lowest.filter(|node| node.get_distance(checkpoint_level).is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::tile::TileKind;

    #[test]
    fn test_nodes_get_node_from_maze() {
        let nodes = Nodes::new(&vec![
            vec![TileKind::Entrypoint, TileKind::Empty],
            vec![TileKind::Wall, TileKind::Checkpoint { level: 1 }],
        ]);

        let node = nodes.get_node(&(1, 1));

        assert_eq!(node, &Node::new(TileKind::Checkpoint { level: 1 }, (1, 1)))
    }

    #[test]
    #[should_panic]
    fn test_nodes_get_node_out_of_bounds() {
        let nodes = Nodes::new(&vec![
            vec![TileKind::Entrypoint, TileKind::Empty],
            vec![TileKind::Wall, TileKind::Checkpoint { level: 1 }],
        ]);

        nodes.get_node(&(5, 1));
    }

    #[test]
    fn test_nodes_get_neighbors_on_the_edge_of_maze() {
        let nodes = Nodes::new(&vec![
            vec![TileKind::Entrypoint, TileKind::Empty],
            vec![TileKind::Wall, TileKind::Checkpoint { level: 1 }],
        ]);

        let neighbors = nodes.get_neighbors_positions(&(0, 1));

        assert_eq!(neighbors, vec![(0, 0), (1, 1)])
    }

    #[test]
    fn test_nodes_get_neighbors_in_the_center_of_maze() {
        let nodes = Nodes::new(&vec![
            vec![TileKind::Empty, TileKind::Empty, TileKind::Empty],
            vec![TileKind::Empty, TileKind::Empty, TileKind::Empty],
            vec![TileKind::Empty, TileKind::Empty, TileKind::Empty],
        ]);

        let neighbors = nodes.get_neighbors_positions(&(1, 1));

        assert_eq!(neighbors, vec![(1, 2), (1, 0), (2, 1), (0, 1),])
    }
}
