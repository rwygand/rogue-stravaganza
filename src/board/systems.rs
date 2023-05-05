use crate::board::components::*;
use crate::vectors::Vector2Int;
use bevy::prelude::*;
use std::collections::HashMap;

use super::components::{Position, Tile};
use super::CurrentBoard;

pub fn spawn_map(mut commands: Commands, mut current: ResMut<CurrentBoard>) {
    current.tiles = HashMap::new();
    for x in 0..current.width {
        for y in 0..current.height {
            let v = Vector2Int::new(x, y);
            let kind = get_tile_at_index(x, y, current.width, current.height);
            let tile = commands.spawn((Position { v }, Tile { kind })).id();
            current.tiles.insert(v, tile);
        }
    }
}

fn get_tile_at_index(x: i32, y: i32, width: i32, height: i32) -> TileType {
    if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
        TileType::Wall
    } else if x == width / 2 && y == height / 2 + 2 {
        TileType::Tree
    } else {
        TileType::Floor
    }
}
