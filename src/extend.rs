
use amethyst::ecs::components::{LocalTransform, Renderable, Transform};
use specs::{Entities, Fetch, Join, System, ReadStorage, WriteStorage};

use components::*;

#[derive(Default)]
pub struct ExtendSystem {
    entities: u64,
}
impl<'a> System<'a> for ExtendSystem {
    type SystemData = (
        // Resources
        Entities<'a>,
        Fetch<'a, Tick>,

        // Components
        WriteStorage<'a, Extension>,
        WriteStorage<'a, LocalTransform>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Tile>,
        WriteStorage<'a, Direction>,
        WriteStorage<'a, Parent>,
        WriteStorage<'a, Snake>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, tick, mut extensions, mut locals, mut globals, mut renderables, mut tiles, mut directions, mut parents, mut snakes) = data;

        if tick.ticked {
            let mut remove = Vec::new();
            for (entity, snake, extend, _, _, _) in (
                &*entities, 
                &mut snakes,
                &mut extensions,
                &tiles.check(), 
                &directions.check(),
                &renderables.check(),
            ).join() {
                let mut current = match snake.end {
                    Some(end) => end,
                    None => entity,
                };

                let extend_entity = entities.create();
                self.entities += 1;
                let parent_tile = tiles.get(current).unwrap().clone();
                let parent_direction = directions.get(current).unwrap().clone();
                let parent_renderable = renderables.get(current).unwrap().clone();

                locals.insert(extend_entity, LocalTransform::default());
                globals.insert(extend_entity, Transform::default());
                renderables.insert(extend_entity, parent_renderable);
                tiles.insert(extend_entity, Tile {
                    x: parent_tile.x - parent_direction.direction.0 as i32,
                    y: parent_tile.y - parent_direction.direction.1 as i32,
                });
                directions.insert(extend_entity, Direction {
                    direction: (0, 0),
                    previous: None,
                });
                parents.insert(extend_entity, Parent(current));

                extend.0 -= 1;
                if extend.0 == 0 {
                    remove.push(entity);
                }

                snake.end = Some(extend_entity);
            }

            for entity in remove {
                extensions.remove(entity);
            }

            if tick.ticks % 100 == 0 {
                for (entity, _) in (&*entities, &snakes.check()).join() {
                    //extensions.insert(entity, Extension(500));
                }
            }
        }
    }
}
