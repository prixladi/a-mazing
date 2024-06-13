use std::collections::HashMap;

use maze_core::{Position, TileKind};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Node {
    kind: TileKind,
    distances: HashMap<i32, u32>,
    position: Position,
}

impl Node {
    pub(crate) fn new(kind: TileKind, position: Position) -> Self {
        Self {
            kind,
            position,
            distances: HashMap::new(),
        }
    }

    pub(crate) fn can_enter(&self) -> bool {
        self.kind != TileKind::Wall
    }

    pub(crate) fn is_checkpoint(&self, checkpoint_level: i32) -> bool {
        match self.kind {
            TileKind::Checkpoint { level } if level == checkpoint_level => true,
            _ => false,
        }
    }

    pub(crate) fn is_entrypoint(&self) -> bool {
        self.kind == TileKind::Entrypoint
    }

    pub(crate) fn has_distance(&self, checkpoint_level: i32) -> bool {
        self.distances.contains_key(&checkpoint_level)
    }

    pub(crate) fn get_distance(&self, checkpoint_level: i32) -> Option<u32> {
        self.distances.get(&checkpoint_level).copied()
    }

    pub(crate) fn set_distance(&mut self, checkpoint_level: i32, distance: u32) {
        self.distances.insert(checkpoint_level, distance);
    }

    pub(crate) fn set_distance_if_not_set(&mut self, checkpoint_level: i32, distance: u32) {
        if !self.has_distance(checkpoint_level) {
            self.set_distance(checkpoint_level, distance);
        }
    }

    pub(crate) fn get_position(&self) -> &Position {
        &self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_node_methods() {
        let node = Node::new(TileKind::Empty, Position { x: 1, y: 1 });

        assert_eq!(node.get_position(), &Position { x: 1, y: 1 });
        assert_eq!(node.can_enter(), true);
        assert_eq!(node.has_distance(1), false);
        assert_eq!(node.get_distance(1), None);
        assert_eq!(node.is_checkpoint(1), false);
        assert_eq!(node.is_entrypoint(), false);
    }

    #[test]
    fn test_entrypoint_node_methods() {
        let node = Node::new(TileKind::Entrypoint, Position { x: 2, y: 2 });

        assert_eq!(node.get_position(), &Position { x: 2, y: 2 });
        assert_eq!(node.can_enter(), true);
        assert_eq!(node.has_distance(1), false);
        assert_eq!(node.get_distance(1), None);
        assert_eq!(node.is_checkpoint(1), false);
        assert_eq!(node.is_entrypoint(), true);
    }

    #[test]
    fn test_checkpoint_node_methods() {
        let node = Node::new(TileKind::Checkpoint { level: 1 }, Position { x: 2, y: 2 });

        assert_eq!(node.get_position(), &Position { x: 2, y: 2 });
        assert_eq!(node.can_enter(), true);
        assert_eq!(node.has_distance(1), false);
        assert_eq!(node.get_distance(1), None);
        assert_eq!(node.is_checkpoint(1), true);
        assert_eq!(node.is_checkpoint(2), false);
        assert_eq!(node.is_entrypoint(), false);
    }

    #[test]
    fn test_node_wall_node_methods() {
        let node = Node::new(TileKind::Wall, Position { x: 2, y: 2 });

        assert_eq!(node.get_position(), &Position { x: 2, y: 2 });
        assert_eq!(node.can_enter(), false);
        assert_eq!(node.has_distance(1), false);
        assert_eq!(node.get_distance(1), None);
        assert_eq!(node.is_checkpoint(1), false);
        assert_eq!(node.is_entrypoint(), false);
    }

    #[test]
    fn test_node_distance_mutation_methods() {
        let mut node = Node::new(TileKind::Empty, Position { x: 2, y: 2 });

        assert_eq!(node.has_distance(1), false);
        assert_eq!(node.get_distance(1), None);
        node.set_distance(1, 5);
        assert_eq!(node.has_distance(1), true);
        assert_eq!(node.get_distance(1), Some(5));
        assert_eq!(node.has_distance(2), false);
        assert_eq!(node.get_distance(2), None);
    }
}
