
use amethyst::ecs::components::LocalTransform;
use amethyst::ecs::resources::{Camera, Projection, ScreenDimensions};
use specs::{Entities, Fetch, FetchMut, Join, System, ReadStorage, WriteStorage};

use components::{Tile, Tick};
use ::TILE_SIZE;

pub struct TileSystem(pub f32);
impl<'a> System<'a> for TileSystem {
    type SystemData = (
        // Resources
        Entities<'a>,
        Fetch<'a, ScreenDimensions>,
        Fetch<'a, Tick>,
        FetchMut<'a, Camera>,

        // Components
        WriteStorage<'a, Tile>,
        WriteStorage<'a, LocalTransform>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, dimensions, tick, camera, mut tiles, mut locals) = data;

        if tick.ticked {
            // Update camera aspect ratio
            match camera.proj {
                Projection::Orthographic { left, mut right, mut bottom, top, near, far, }=> {
                    right = dimensions.w;
                    bottom = dimensions.h;
                },
                _ => { },
            }

            // Fix perspective for proper scaling
            for (tile, locals) in (&mut tiles, &mut locals).join() {
                locals.translation[0] = tile.x as f32 * (TILE_SIZE / 2.0) + (TILE_SIZE / 4.0);
                locals.translation[1] = tile.y as f32 * (TILE_SIZE / 2.0) + (TILE_SIZE / 4.0);
                locals.scale[0] = TILE_SIZE / 2.0 - 1.5;
                locals.scale[1] = TILE_SIZE / 2.0 - 1.5;
            }
        }
    }
}
