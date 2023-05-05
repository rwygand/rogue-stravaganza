use bevy::prelude::*;
use std::collections::HashMap;

use crate::states::MainState;
use crate::vectors::Vector2Int;

pub mod components;
mod systems;

pub const BOARD_SIZE: IVec2 = IVec2::new(80, 50);

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentBoard {
            width: BOARD_SIZE.x,
            height: BOARD_SIZE.y,
            ..default()
        })
        .add_system(systems::spawn_map.in_schedule(OnEnter(MainState::Game)));
    }
}

#[derive(Resource)]
pub struct CurrentBoard {
    pub tiles: HashMap<Vector2Int, Entity>,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
}

impl Default for CurrentBoard {
    fn default() -> Self {
        CurrentBoard {
            tiles: HashMap::new(),
            width: 16,
            height: 16,
            depth: 1,
        }
    }
}
