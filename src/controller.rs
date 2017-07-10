

use amethyst::ecs::resources::InputHandler;
use specs::{Entities, Fetch, FetchMut, Join, System, ReadStorage, WriteStorage};

use components::{Controls, Direction, Grid, Parent, Tile, Type, Snake, Tick};

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
        ReadStorage<'a, Type>,
        ReadStorage<'a, Parent>,
        ReadStorage<'a, Controls>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, input, tick, mut grid, mut tiles, mut snakes, mut directions, types, parents, controls) = data;

        for (direction, snake, tile, controls) in (&mut directions, &mut snakes, &mut tiles, &controls).join() {
            // Figure out a valid direction for the snake.
            match (
                input.button_down(controls.left),
                input.button_down(controls.right),
                input.button_down(controls.up),
                input.button_down(controls.down),
            ) {
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
            // Update follower directions.
            for (entity, parent, _) in (&*entities, &parents, &directions.check()).join() {
                let parent_direction = match directions.get(parent.0) {
                    Some(direction) => Some(direction.clone()),
                    None => None,
                };

                if let Some(parent_direction) = parent_direction {
                    let mut direction = directions.get_mut(entity).unwrap();
                    direction.previous = Some(direction.direction);

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

                if let Some(entity) = grid.get(tile.x as usize, tile.y as usize) {
                    // Check if the entity kills or is eatable.
                    if let Some(t) = types.get(entity) {
                        match *t {
                            Type::Kill => println!("Game over"), // TODO: Stop the game.
                            Type::Eat => println!("Food"), // TODO: Implement extending the snake and score.
                        }
                    }
                }
                
                // Add entity to its new tile.
                grid.set(tile.x as usize, tile.y as usize, Some(entity));
            }
        }
    }
}
