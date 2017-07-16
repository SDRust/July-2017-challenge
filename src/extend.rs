
use amethyst::ecs::components::{LocalTransform, Renderable, Transform};
use specs::{Entities, Fetch, Join, System, WriteStorage};

use components::*;

#[derive(Default)]
pub struct ExtendSystem;
impl<'a> System<'a> for ExtendSystem {
    type SystemData = (
        // Resources
        Entities<'a>,
        Fetch<'a, Tick>,

        // Components
        WriteStorage<'a, Type>,
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
        // Destructure the `Self::SystemData` into something more usable.
        let (
            entities, 
            tick, 
            mut types,
            mut extensions, 
            mut locals, 
            mut globals, 
            mut renderables, 
            mut tiles, 
            mut directions, 
            mut parents, 
            mut snakes
        ) = data;

        if tick.ticked {
            let mut remove = Vec::new();
            
            // Iterate over any entity that is a snake and needs to be extended.
            for (entity, snake, extend, _, _, _) in (
                &*entities, 
                &mut snakes,
                &mut extensions,

                // Only check that the entity has the component, but without borrowing the storage.
                &tiles.check(), 
                &directions.check(),
                &renderables.check(),
            ).join() {
                let current = match snake.end {
                    Some(end) => end,
                    None => entity,
                };

                // Create a "tail" entity.
                let extend_entity = entities.create();
                let parent_tile = tiles.get(current).unwrap().clone();
                let parent_direction = directions.get(current).unwrap().clone();
                let parent_renderable = renderables.get(current).unwrap().clone();

                // Somewhat unfortunate that we have to borrow these storages mutably to insert.
                // Eventually use a lazy insertion so the storages don't block other systems.
                types.insert(extend_entity, Type::Kill);
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

            // Clean up the extension component when it hits 0.
            for entity in remove {
                extensions.remove(entity);
            }
        }
    }
}
