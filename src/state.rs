
use amethyst::asset_manager::AssetManager;
use amethyst::ecs::components::{Mesh, LocalTransform, Texture, Transform};
use amethyst::ecs::resources::Button;
use amethyst::renderer::{VertexPosNormal, Pipeline};
use amethyst::{Event, State, Trans, VirtualKeyCode, WindowEvent};
use specs::{Entity, World};

use components::{Controls, Direction, Extension, Tile, Snake, Type};

pub struct GameState;
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
            let aspect_ratio = dim.aspect_ratio;
            let eye = [0., 0., 1.0];
            let target = [0., 0., 0.];
            let up = [0., 1., 0.];

            let left = 0.0;
            let right = dim.w;
            let top = 0.0;

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

        // Add all resources
        //world.add_resource::<Score>(Score::new());
        world.add_resource::<InputHandler>(InputHandler::new());

        // Generate a square mesh
        assets.register_asset::<Mesh>();
        assets.register_asset::<Texture>();

        // Textures
        assets.load_asset_from_data::<Texture, [f32; 4]>("white", [1.0, 1.0, 1.0, 1.0]);
        assets.load_asset_from_data::<Texture, [f32; 4]>("blue", [0.0, 0.0, 1.0, 1.0]);

        // Square vertices/mesh/polygon
        let square_verts = gen_rectangle(1.0, 1.0);
        assets.load_asset_from_data::<Mesh, Vec<VertexPosNormal>>("square", square_verts);

        let player1 = assets.create_renderable("square", "white", "white", "white", 1.0).unwrap();
        let player2 = assets.create_renderable("square", "blue", "blue", "blue", 1.0).unwrap();

        // Set up entities
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
            .with(Extension(5)) // Start the snake off with 6 pieces
            .build();

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
            .with(Direction::default())
            .with(Controls {
                left: VirtualKeyCode::Left.into(),
                right: VirtualKeyCode::Right.into(),
                up: VirtualKeyCode::Up.into(),
                down: VirtualKeyCode::Down.into(),
            })
            .with(Extension(5)) // Start the snake off with 6 pieces
            .build();
    }

    fn handle_events(&mut self,
                     events: &[WindowEvent],
                     world: &mut World,
                     _: &mut AssetManager,
                     _: &mut Pipeline)
                     -> Trans {
        use amethyst::ecs::resources::InputHandler;

        let mut input = world.write_resource::<InputHandler>();
        input.update(events);

        for e in events {
            match **e {
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) |
                Event::Closed => return Trans::Quit,
                _ => (),
            }
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
