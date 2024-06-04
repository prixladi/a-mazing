use crate::{core::tile::TileKind, Position};

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    kind: TileKind,
    distance: Option<u32>,
    position: Position,
}

impl Node {
    pub fn new(kind: TileKind, position: Position) -> Self {
        Self {
            kind,
            position,
            distance: None,
        }
    }

    pub fn can_enter(&self) -> bool {
        self.kind != TileKind::Wall
    }

    pub fn is_exit(&self) -> bool {
        self.kind == TileKind::Exit
    }

    pub fn is_entrance(&self) -> bool {
        self.kind == TileKind::Entrance
    }

    pub fn has_distance(&self) -> bool {
        self.distance.is_some()
    }

    pub fn get_distance(&self) -> Option<u32> {
        self.distance
    }

    pub fn set_distance(&mut self, distance: u32) {
        self.distance = Some(distance);
    }

    pub fn get_position(&self) -> Position {
        self.position
    }
}

#[derive(Debug)]
pub struct Nodes {
    data: Vec<Vec<Node>>,
}

impl Nodes {
    pub fn new(nodes: Vec<Vec<Node>>) -> Self {
        Self { data: nodes }
    }

    pub fn get_node(&self, (x, y): &Position) -> &Node {
        &self.data[*x][*y]
    }

    pub fn get_node_mut(&mut self, (x, y): &Position) -> &mut Node {
        &mut self.data[*x][*y]
    }

    pub fn get_neighbors(&self, (x, y): &Position) -> Vec<&Node> {
        let mut neighbors = Vec::with_capacity(4);

        if y + 1 < self.data[0].len() {
            neighbors.push(self.get_node(&(*x, y + 1)));
        }
        if *y > 0 {
            neighbors.push(self.get_node(&(*x, y - 1)));
        }

        if x + 1 < self.data[0].len() {
            neighbors.push(self.get_node(&(x + 1, *y)));
        }
        if *x > 0 {
            neighbors.push(self.get_node(&(x - 1, *y)));
        }

        return neighbors;
    }

    pub fn get_lowest_distance_neighbor(&self, position: &Position) -> Option<&Node> {
        let mut lowest: Option<&Node> = None;

        for neighbor in self.get_neighbors(position) {
            lowest = lowest
                .and_then(|node| node.get_distance())
                .and_then(|distance| {
                    neighbor
                        .get_distance()
                        .and_then(|neighbor_distance| {
                            if distance > neighbor_distance {
                                Some(neighbor)
                            } else {
                                None
                            }
                        })
                        .or(lowest)
                })
                .or(Some(neighbor));
        }

        lowest.filter(|node| node.get_distance().is_some())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_node_methods_1() {
        let node = Node::new(TileKind::Empty, (1, 1));

        assert_eq!(node.get_position(), (1, 1));
        assert_eq!(node.can_enter(), true);
        assert_eq!(node.has_distance(), false);
        assert_eq!(node.get_distance(), None);
        assert_eq!(node.is_exit(), false);
        assert_eq!(node.is_entrance(), false);
    }

    #[test]
    fn test_node_methods_2() {
        let node = Node::new(TileKind::Entrance, (2, 2));

        assert_eq!(node.get_position(), (2, 2));
        assert_eq!(node.can_enter(), true);
        assert_eq!(node.has_distance(), false);
        assert_eq!(node.get_distance(), None);
        assert_eq!(node.is_exit(), false);
        assert_eq!(node.is_entrance(), true);
    }

    #[test]
    fn test_node_methods_3() {
        let node = Node::new(TileKind::Exit, (2, 2));

        assert_eq!(node.get_position(), (2, 2));
        assert_eq!(node.can_enter(), true);
        assert_eq!(node.has_distance(), false);
        assert_eq!(node.get_distance(), None);
        assert_eq!(node.is_exit(), true);
        assert_eq!(node.is_entrance(), false);
    }

    #[test]
    fn test_node_methods_4() {
        let node = Node::new(TileKind::Wall, (2, 2));

        assert_eq!(node.get_position(), (2, 2));
        assert_eq!(node.can_enter(), false);
        assert_eq!(node.has_distance(), false);
        assert_eq!(node.get_distance(), None);
        assert_eq!(node.is_exit(), false);
        assert_eq!(node.is_entrance(), false);
    }

    #[test]
    fn test_node_methods_5() {
        let mut node = Node::new(TileKind::Empty, (2, 2));

        assert_eq!(node.has_distance(), false);
        assert_eq!(node.get_distance(), None);
        node.set_distance(5);
        assert_eq!(node.has_distance(), true);
        assert_eq!(node.get_distance(), Some(5));
    }

    #[test]
    fn test_nodes_get_node_valid() {
        let nodes = Nodes::new(vec![
            vec![
                Node::new(TileKind::Entrance, (0, 0)),
                Node::new(TileKind::Empty, (0, 1)),
            ],
            vec![
                Node::new(TileKind::Wall, (1, 0)),
                Node::new(TileKind::Exit, (1, 1)),
            ],
        ]);

        let node = nodes.get_node(&(1, 1));

        assert_eq!(node, &Node::new(TileKind::Exit, (1, 1)))
    }

    #[test]
    #[should_panic]
    fn test_nodes_get_node_invalid() {
        let nodes = Nodes::new(vec![
            vec![
                Node::new(TileKind::Entrance, (0, 0)),
                Node::new(TileKind::Empty, (0, 1)),
            ],
            vec![
                Node::new(TileKind::Wall, (1, 0)),
                Node::new(TileKind::Exit, (1, 1)),
            ],
        ]);

        nodes.get_node(&(5, 1));
    }

    #[test]
    fn test_nodes_get_neighbors_0() {
        let nodes = Nodes::new(vec![
            vec![
                Node::new(TileKind::Entrance, (0, 0)),
                Node::new(TileKind::Empty, (0, 1)),
            ],
            vec![
                Node::new(TileKind::Wall, (1, 0)),
                Node::new(TileKind::Exit, (1, 1)),
            ],
        ]);

        let neighbors = nodes.get_neighbors(&(0, 1));

        assert_eq!(
            neighbors,
            vec![
                &Node::new(TileKind::Entrance, (0, 0)),
                &Node::new(TileKind::Exit, (1, 1))
            ]
        )
    }

    #[test]
    fn test_nodes_get_neighbors_1() {
        let nodes = Nodes::new(vec![
            vec![
                Node::new(TileKind::Empty, (0, 0)),
                Node::new(TileKind::Empty, (0, 1)),
                Node::new(TileKind::Empty, (0, 2)),
            ],
            vec![
                Node::new(TileKind::Empty, (1, 0)),
                Node::new(TileKind::Empty, (1, 1)),
                Node::new(TileKind::Empty, (1, 2)),
            ],
            vec![
                Node::new(TileKind::Empty, (2, 0)),
                Node::new(TileKind::Empty, (2, 1)),
                Node::new(TileKind::Empty, (2, 2)),
            ],
        ]);

        let neighbors = nodes.get_neighbors(&(1, 1));

        assert_eq!(
            neighbors,
            vec![
                &Node::new(TileKind::Empty, (1, 2)),
                &Node::new(TileKind::Empty, (1, 0)),
                &Node::new(TileKind::Empty, (2, 1)),
                &Node::new(TileKind::Empty, (0, 1)),
            ]
        )
    }
}
