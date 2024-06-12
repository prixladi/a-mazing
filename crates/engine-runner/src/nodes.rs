use engine_core::{Position, TileBoard};

use super::node::Node;

#[derive(Debug, Clone)]
pub(crate) struct Nodes {
    data: Vec<Vec<Node>>,
}

impl Nodes {
    pub(crate) fn new(tiles: &TileBoard) -> Self {
        let nodes = tiles
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(|(y, kind)| Node::new(*kind, Position { x, y }))
                    .collect()
            })
            .collect();

        Self { data: nodes }
    }

    pub(crate) fn get_node(&self, position: &Position) -> &Node {
        &self.data[position.x][position.y]
    }

    pub(crate) fn get_node_mut(&mut self, position: &Position) -> &mut Node {
        &mut self.data[position.x][position.y]
    }

    pub(crate) fn get_neighbors_positions(&self, &Position { x, y }: &Position) -> Vec<Position> {
        let mut neighbors = Vec::with_capacity(4);

        if y + 1 < self.data[0].len() {
            neighbors.push(Position { x, y: y + 1 });
        }
        if y > 0 {
            neighbors.push(Position { x, y: y - 1 });
        }

        if x + 1 < self.data.len() {
            neighbors.push(Position { x: x + 1, y });
        }
        if x > 0 {
            neighbors.push(Position { x: x - 1, y });
        }

        return neighbors;
    }

    pub(crate) fn get_lowest_distance_neighbor(
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

    use engine_core::TileKind;

    #[test]
    fn test_nodes_get_node_from_maze() {
        let nodes = Nodes::new(&vec![
            vec![TileKind::Entrypoint, TileKind::Empty],
            vec![TileKind::Wall, TileKind::Checkpoint { level: 1 }],
        ]);

        let node = nodes.get_node(&Position { x: 1, y: 1 });

        assert_eq!(
            node,
            &Node::new(TileKind::Checkpoint { level: 1 }, Position { x: 1, y: 1 })
        )
    }

    #[test]
    #[should_panic]
    fn test_nodes_get_node_out_of_bounds() {
        let nodes = Nodes::new(&vec![
            vec![TileKind::Entrypoint, TileKind::Empty],
            vec![TileKind::Wall, TileKind::Checkpoint { level: 1 }],
        ]);

        nodes.get_node(&Position { x: 5, y: 1 });
    }

    #[test]
    fn test_nodes_get_neighbors_on_the_edge_of_maze() {
        let nodes = Nodes::new(&vec![
            vec![TileKind::Entrypoint, TileKind::Empty],
            vec![TileKind::Wall, TileKind::Checkpoint { level: 1 }],
        ]);

        let neighbors = nodes.get_neighbors_positions(&Position { x: 0, y: 1 });

        assert_eq!(
            neighbors,
            vec![Position { x: 0, y: 0 }, Position { x: 1, y: 1 }]
        )
    }

    #[test]
    fn test_nodes_get_neighbors_in_the_center_of_maze() {
        let nodes = Nodes::new(&vec![
            vec![TileKind::Empty, TileKind::Empty, TileKind::Empty],
            vec![TileKind::Empty, TileKind::Empty, TileKind::Empty],
            vec![TileKind::Empty, TileKind::Empty, TileKind::Empty],
        ]);

        let neighbors = nodes.get_neighbors_positions(&Position { x: 1, y: 1 });

        assert_eq!(
            neighbors,
            vec![
                Position { x: 1, y: 2 },
                Position { x: 1, y: 0 },
                Position { x: 2, y: 1 },
                Position { x: 0, y: 1 },
            ]
        )
    }
}
