use specs::{System, ReadStorage};
use ggez::Context;

use components::{Position, Tile};

struct Renderer<'c> {
    ctx: &'c Context
}

impl<'a, 'c> System<'a> for Renderer<'c> {
    type SystemData = (ReadStorage<'a, Position>,
                       ReadStorage<'a, Tile>);

    fn run(&mut self, (pos, tile): Self::SystemData) {
        use specs::Join;
        for (pos, tile) in (&pos, &tile).join() {
            unimplemented!();
        }
    }
}