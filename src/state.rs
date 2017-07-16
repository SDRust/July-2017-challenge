
use amethyst::asset_manager::AssetManager;
use amethyst::ecs::components::{Mesh, LocalTransform, Texture, Transform};
use amethyst::renderer::{VertexPosNormal, Pipeline};
use amethyst::{Event, State, Trans, VirtualKeyCode, WindowEvent};
use specs::{World, Join};

use components::{Controls, Direction, Extension, Grid, Tile, Snake, Type, Tick};

pub struct GameState;

impl GameState {
    fn reset(&mut self, world: &mut World, assets: &mut AssetManager) {
        // Reset any previous game state (in case game is restarting).
        {
            // Remove all previous entities.
            for entity in world.entities().join()
            {
                world.entities().delete(entity);
            }
            // World::maintain will clean up the entities immediately.
            // (Otherwise weird stuff happens?)
            world.maintain();

            // Reset tick and grid state.
            let mut tick = world.write_resource::<Tick>();
            *tick = Tick::default();

            let mut grid = world.write_resource::<Grid>();
            *grid = Grid::new(::GRID_X, ::GRID_Y);
        }

        let player1 = assets.create_renderable("square", "white", "white", "white", 1.0).unwrap();

        // Set up snakes
        world.create_entity()
            .with(player1.clone())
            .with(LocalTransform::default())
            .with(Transform::default())
            .with(Tile { x: (::GRID_X / 2 - ::GRID_X / 4) as i32, y: ( 3 * ::GRID_Y / 4) as i32 })
            .with(Snake { 
                queued_direction: (0, -1),
                length: 1,
                end: None,
            })
            .with(Type::Kill)
            .with(Direction::default())
            .with(Controls {
                left: VirtualKeyCode::A.into(),
                right: VirtualKeyCode::D.into(),
                up: VirtualKeyCode::W.into(),
                down: VirtualKeyCode::S.into(),
            })
            .with(Extension(6)) // Start the snake off with 7 pieces (head + 6 tail pieces)
            .build();

        /*
        // Extra snake if you want to add it.
        let player2 = assets.create_renderable("square", "blue", "blue", "blue", 1.0).unwrap();
        world.create_entity()
            .with(player2.clone())
            .with(LocalTransform::default())
            .with(Transform::default())
            .with(Tile { x: (::GRID_X / 2 + ::GRID_X / 4) as i32, y: ( 3 * ::GRID_Y / 4) as i32 })
            .with(Snake { 
                queued_direction: (0, -1),
                length: 1,
                end: None,
            })
            .with(Type::Kill)
            .with(Direction::default())
            .with(Controls {
                left: VirtualKeyCode::Left.into(),
                right: VirtualKeyCode::Right.into(),
                up: VirtualKeyCode::Up.into(),
                down: VirtualKeyCode::Down.into(),
            })
            .with(Extension(200)) // Start the snake off with 6 pieces
            .build();
        */
    }
}

impl State for GameState {
    fn on_start(&mut self, world: &mut World, assets: &mut AssetManager, pipe: &mut Pipeline) {
        use amethyst::ecs::resources::{Camera, InputHandler, Projection, ScreenDimensions};
        use amethyst::renderer::Layer;
        use amethyst::renderer::pass::{Clear, DrawFlat};

        let layer = Layer::new("main",
                               vec![Clear::new([0.0, 0.0, 0.0, 1.0]),
                                    DrawFlat::new("main", "main")]);

        pipe.layers.push(layer);

        // Camera
        {
            let dim = world.read_resource::<ScreenDimensions>();
            let mut camera = world.write_resource::<Camera>();
            let eye = [0., 0., 1.0];
            let target = [0., 0., 0.];
            let up = [0., 1., 0.];

            let proj = Projection::Orthographic {
                left: 0.0,
                right: dim.w,
                bottom: dim.h,
                top: 0.0,
                near: -1.0,
                far: 1.0,
            };

            camera.proj = proj;
            camera.eye = eye;
            camera.target = target;
            camera.up = up;
        }

        // Generate a square mesh
        assets.register_asset::<Mesh>();
        assets.register_asset::<Texture>();

        // Textures
        assets.load_asset_from_data::<Texture, [f32; 4]>("white", [1.0, 1.0, 1.0, 1.0]);
        assets.load_asset_from_data::<Texture, [f32; 4]>("blue", [0.0, 0.0, 1.0, 1.0]);
        assets.load_asset_from_data::<Texture, [f32; 4]>("red", [1.0, 0.0, 0.0, 1.0]);

        // Square vertices/mesh/polygon
        let square_verts = gen_rectangle(1.0, 1.0);
        assets.load_asset_from_data::<Mesh, Vec<VertexPosNormal>>("square", square_verts);

        // Add all resources
        world.add_resource::<InputHandler>(InputHandler::new());

        self.reset(world, assets);
    }

    fn handle_events(&mut self,
                     events: &[WindowEvent],
                     world: &mut World,
                     assets: &mut AssetManager,
                     _: &mut Pipeline)
                     -> Trans {
        use amethyst::ElementState;
        use amethyst::ecs::resources::InputHandler;

        {
            let mut input = world.write_resource::<InputHandler>();
            input.update(events);
        }

        // Press R to restart game.
        for e in events {
            match **e {
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::R)) =>
                    self.reset(world, assets),
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) |
                Event::Closed => return Trans::Quit,
                _ => (),
            }
        }

        Trans::None
    }

    fn update(&mut self, world: &mut World, assets: &mut AssetManager, _: &mut Pipeline) -> Trans {
        use components::{Tick, Grid};

        // Spawn a new food every 50 ticks.
        let pos =
        {
            let ticks = world.read_resource::<Tick>();
            
            if ticks.ticks % 50 == 0 && ticks.ticked {
                let grid = world.read_resource::<Grid>();
            
                // TODO: Get a random empty position for the food.
                use rand::{self, Rng};
                let mut rng = rand::thread_rng();
                let x: i32 = rng.gen_range(0, grid.len.0 as i32);
                let y: i32 = rng.gen_range(0, grid.len.1 as i32);                
                Some((x, y))
            }
            else {
                None
            }
        };

        if let Some(position) = pos {
            let food = assets.create_renderable("square", "red", "red", "red", 1.0).unwrap();

            world.create_entity()
                .with(food)
                .with(LocalTransform::default())
                .with(Transform::default())
                .with(Direction {
                    direction: (0, 0),
                    previous: None,
                })
                .with(Tile { x: position.0 as i32, y: position.1 as i32 })
                .with(Type::Eat)
                .build();
        }
        
        Trans::None
    }
}

// Generate a suqare from vertices
fn gen_rectangle(w: f32, h: f32) -> Vec<VertexPosNormal> {
    let data: Vec<VertexPosNormal> = vec![VertexPosNormal {
                                              pos: [-w / 2., -h / 2., 0.],
                                              normal: [0., 0., 1.],
                                              tex_coord: [0., 0.],
                                          },
                                          VertexPosNormal {
                                              pos: [w / 2., -h / 2., 0.],
                                              normal: [0., 0., 1.],
                                              tex_coord: [1., 0.],
                                          },
                                          VertexPosNormal {
                                              pos: [w / 2., h / 2., 0.],
                                              normal: [0., 0., 1.],
                                              tex_coord: [1., 1.],
                                          },
                                          VertexPosNormal {
                                              pos: [w / 2., h / 2., 0.],
                                              normal: [0., 0., 1.],
                                              tex_coord: [1., 1.],
                                          },
                                          VertexPosNormal {
                                              pos: [-w / 2., h / 2., 0.],
                                              normal: [0., 0., 1.],
                                              tex_coord: [1., 1.],
                                          },
                                          VertexPosNormal {
                                              pos: [-w / 2., -h / 2., 0.],
                                              normal: [0., 0., 1.],
                                              tex_coord: [1., 1.],
                                          }];
    data
}
