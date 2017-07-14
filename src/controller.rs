

use amethyst::ecs::resources::InputHandler;
use specs::{Entities, Fetch, FetchMut, Join, System, ReadStorage, WriteStorage};

use components::{Controls, Direction, Extension, Grid, Parent, Tile, Type, Snake, Tick};

#[derive(Default)]
pub struct ControllerSystem;
impl<'a> System<'a> for ControllerSystem {
    type SystemData = (
        // Resources
        Entities<'a>,
        Fetch<'a, InputHandler>,
        Fetch<'a, Tick>,
        FetchMut<'a, Grid>,

        // Components
        WriteStorage<'a, Tile>,
        WriteStorage<'a, Snake>,
        WriteStorage<'a, Direction>,
        WriteStorage<'a, Extension>,
        ReadStorage<'a, Type>,
        ReadStorage<'a, Parent>,
        ReadStorage<'a, Controls>,
    );
    fn run(&mut self, data: Self::SystemData) {
        // Destructure the `Self::SystemData` into something more usable.
        let (
            entities, 
            input, 
            tick, 
            mut grid, 
            mut tiles, 
            mut snakes, 
            mut directions, 
            mut extensions,
            types, 
            parents, 
            controls,
        ) = data;

        // Iterate over snakes.
        for (direction, snake, tile, controls) in (&mut directions, &mut snakes, &mut tiles, &controls).join() {
            // Figure out a valid direction for the snake.
            match (
                input.button_down(controls.left),
                input.button_down(controls.right),
                input.button_down(controls.up),
                input.button_down(controls.down),
            ) {
                // Queue the direction for the next tick
                (true, _, _, _) if direction.direction.0 != 1 => snake.queued_direction = (-1, 0),
                (_, true, _, _) if direction.direction.0 != -1 => snake.queued_direction = (1, 0),
                (_, _, true, _) if direction.direction.1 != 1 => snake.queued_direction = (0, -1),
                (_, _, _, true) if direction.direction.1 != -1 => snake.queued_direction = (0, 1),
                _ => { },
            }

            if tick.ticked {
                // Update direction to the new queued direction.
                direction.previous = Some(direction.direction);
                direction.direction = snake.queued_direction;
            }
        }

        if tick.ticked {
            // Update follower (tail) directions.
            for (entity, parent, _) in (&*entities, &parents, &directions.check()).join() {
                let parent_direction = match directions.get(parent.0) {
                    Some(direction) => Some(direction.clone()),
                    None => None,
                };

                // Get the directino of the parent.
                if let Some(parent_direction) = parent_direction {
                    let mut direction = directions.get_mut(entity).unwrap();
                    direction.previous = Some(direction.direction);

                    // Use the parent's previous direction so it follows the path the parent took.
                    if let Some(previous) = parent_direction.previous {
                        direction.direction = previous;
                    }
                }
            }

            // Update tiles to correct positions.
            for (entity, direction, tile) in (&*entities, &directions, &mut tiles).join() {
                // Only remove the entity from the previous portion of the grid if it actually this entity
                // in that piece of the grid.
                if let Some(other) = grid.get(tile.x as usize, tile.y as usize) {
                    if other == entity {
                        grid.set(tile.x as usize, tile.y as usize, None);
                    }
                }

                tile.x += direction.direction.0 as i32;
                tile.y += direction.direction.1 as i32;

                // Only if the current tile is a snake.
                if let Some(snake) = snakes.get(entity) {
                    if let Some(other) = grid.get(tile.x as usize, tile.y as usize) {
                        // Check if the entity kills or is eatable.
                        if let Some(t) = types.get(other) {
                            match *t {
                                // TODO: Stop the game.
                                Type::Kill => {
                                    println!("Game over for {:?} killed by {:?}", entity, other);
                                },

                                // TODO: Implement extending the snake and score.
                                Type::Eat => {
                                    println!("Eat {:?}", other);
                                },
                            }
                        }
                    }
                }
                
                // Add entity to its new tile in the grid.
                grid.set(tile.x as usize, tile.y as usize, Some(entity));
            }
        }
    }
}
