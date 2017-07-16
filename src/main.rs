
extern crate amethyst;
extern crate specs;
extern crate rand;

use amethyst::Application;
use amethyst::project::Config;
use amethyst::ecs::World;
use amethyst::ecs::systems::TransformSystem;
use amethyst::gfx_device::DisplayConfig;

use specs::{DispatcherBuilder};

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

pub const TILE_SIZE: f32 = 100.0;
pub const GRID_X: usize = 12; // WINDOW_SIZE / (TILE_SIZE / 2)
pub const GRID_Y: usize = 12;

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
    let dispatcher = DispatcherBuilder::new()
        .add(ControllerSystem::default(), "controller", &[])
        .add(TickSystem::default(), "ticks", &["controller"])
        .add(TileSystem(0.0), "tiles", &[])
        .add(TransformSystem::new(), "transform", &["tiles"])
        .add(ExtendSystem::default(), "extend", &["tiles", "ticks"])
        .build();

    let mut game = Application::new(GameState, dispatcher, world, cfg);
    game.run();
}

