use crate::board::components::*;
use crate::pieces::components::Occupier;
use crate::vectors::Vector2Int;
use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;

use super::components::{Position, Tile};
use super::CurrentBoard;

pub fn spawn_map(
    mut commands: Commands,
    mut current: ResMut<CurrentBoard>,
    rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    current.tiles = HashMap::new();
    let rng = rng.into_inner();
    for x in 0..current.width {
        for y in 0..current.height {
            let v = Vector2Int::new(x, y);
            let kind = get_tile_at_index(x, y, current.width, current.height, rng);
            let tile = commands.spawn((Position { v }, Tile { kind })).id();
            if kind != TileType::Floor {
                commands.entity(tile).insert(Occupier {});
            }
            current.tiles.insert(v, tile);
        }
    }
}

fn get_tile_at_index(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    rng: &mut GlobalEntropy<ChaCha8Rng>,
) -> TileType {
    if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
        TileType::Wall
    } else if rng.gen_ratio(1, 20) {
        TileType::Tree
    } else {
        TileType::Floor
    }
}
