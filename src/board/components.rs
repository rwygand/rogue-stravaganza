use bevy::prelude::*;

use crate::vectors::Vector2Int;

#[derive(Component)]
pub struct Position {
    pub v: Vector2Int,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum TileType {
    Floor,
    Wall,
    Tree,
}

#[derive(Component, Debug)]
pub struct Tile {
    pub kind: TileType,
}
