
extern crate amethyst;
extern crate specs;

use amethyst::{Application, Event, State, Trans, VirtualKeyCode, WindowEvent};
use amethyst::asset_manager::AssetManager;
use amethyst::project::Config;
use amethyst::ecs::{Component, Fetch, FetchMut, Join, System, VecStorage, World, WriteStorage};
use amethyst::ecs::components::{Mesh, LocalTransform, Texture, Transform};
use amethyst::ecs::resources::{Camera, InputHandler, Projection, Time};
use amethyst::ecs::systems::TransformSystem;
use amethyst::gfx_device::DisplayConfig;
use amethyst::renderer::{Pipeline, VertexPosNormal};

use specs::{DispatcherBuilder, NullStorage};

use components::*;
use state::GameState;
use tile::TileSystem;
use controller::ControllerSystem;
use tick::TickSystem;
use extend::ExtendSystem;

mod components;
mod state;
mod tile;
mod controller;
mod tick;
mod extend;

pub const TILE_SIZE: f32 = 50.0;
pub const GRID_X: usize = 24; // 600 / 50
pub const GRID_Y: usize = 24;

fn main() {
    let path = format!("{}/resources/config.yml", env!("CARGO_MANIFEST_DIR"));
    let cfg = DisplayConfig::load(path);
    
    let mut world = World::new();
    // Add resources.
    world.add_resource::<Tick>(Tick::default());
    world.add_resource::<Grid>(Grid::new(GRID_X, GRID_Y));

    // Register component.
    world.register::<Controls>();
    world.register::<Snake>();
    world.register::<Type>();
    world.register::<Parent>();
    world.register::<Direction>();
    world.register::<Extension>();
    world.register::<Tile>();

    // Add systems that should be run in parallel.
    let mut dispatcher = DispatcherBuilder::new()
        .add(TickSystem::default(), "ticks", &[])
        .add(TileSystem(0.0), "tiles", &[])
        .add(TransformSystem::new(), "transform", &["tiles"])
        .add(ExtendSystem::default(), "extend", &["tiles", "ticks"])
        .add(ControllerSystem::default(), "controller", &["ticks"])
        .build();

    let mut game = Application::new(GameState, dispatcher, world, cfg);
    game.run();
}

