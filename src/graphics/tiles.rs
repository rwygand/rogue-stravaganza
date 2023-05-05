use bevy::prelude::*;

use super::{GraphicsAssets, TILE_SIZE, TILE_Z};
use crate::board::components::{Position, Tile};
use crate::graphics::assets::sprite_idx_for_tile;

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Tile, &Position), Added<Tile>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, tile, position) in query.iter() {
        let mut sprite = TextureAtlasSprite::new(sprite_idx_for_tile(&tile.kind));

        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        let v = super::get_world_position(position, TILE_Z);
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.sprite_texture.clone(),
            transform: Transform::from_translation(v),
            ..Default::default()
        });
    }
}
