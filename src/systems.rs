use std::collections::HashMap;

use specs::{Entities, Entity, System, Read, Write, ReadStorage};
use ggez::Context;
use ggez::graphics::spritebatch::SpriteBatch;

use components::{Position, Face, Tile};
use constants::GRID_SIZE;
use app::{SpriteId, BackdropId, get_tile_params, add_sprite, draw_spritebatch};
use misc;

#[derive(Default)]
pub struct Camera {
    entity: Option<Entity>
}

struct Renderer<'c> {
    ctx: &'c mut Context
}

impl<'a, 'c> System<'a> for Renderer<'c> {
    type SystemData = (
        Read<'a, Camera>,
        Write<'a, HashMap<SpriteId, SpriteBatch>>,
        Read<'a, HashMap<BackdropId, SpriteBatch>>,
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Face>,
        ReadStorage<'a, Tile>
    );

    fn run(&mut self, (camera, mut sprites, backdrops, entities, pos, face, tile): Self::SystemData) {
        use specs::Join;
        
        let camera = camera.entity.unwrap();
        let camera = pos.get(camera).unwrap();

        let mut sprite_ids = vec![];

        for (entity, pos, tile) in (&*entities, &pos, &tile).join() {
            let x = (pos.x - camera.x) * GRID_SIZE;
            let y = (pos.y - camera.y) * GRID_SIZE;
            let p = get_tile_params(
                &mut self.ctx,
                misc::Position { x: pos.x, y: pos.y },
                misc::Position { x, y },
                face.get(entity).map(|face| face.direction)
            );
            add_sprite(&mut sprites, &tile.sprite, p);
            sprite_ids.push(tile.sprite.clone());
        }

        for sprite_id in sprite_ids.iter() {
            draw_spritebatch(&mut self.ctx, &mut sprites, sprite_id).expect("spritebatch failed to draw");
        }
    }
}