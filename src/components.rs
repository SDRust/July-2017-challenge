
use std::collections::HashSet;

use specs::{Component, Entity, DenseVecStorage, NullStorage};
use amethyst::ecs::resources::Button;

#[derive(Debug, Default)]
pub struct Tick {
    // How many ticks have happened so far
    pub ticks: u64,

    // Did a tick occur this frame.
    pub ticked: bool,
}

pub struct Grid {
    pub taken: HashSet<usize>,
    list: Vec<Option<Entity>>,
    len: (usize, usize),
}
impl Grid {
    pub fn new(x: usize, y: usize) -> Self {
        let mut list = Vec::with_capacity(x * y);
        for _ in 0..(x * y) {
            list.push(None);
        }
        
        Grid {
            taken: HashSet::new(),
            list: list,
            len: (x, y),
        }
    }

    fn bounds(&self, x: usize, y: usize) -> bool {
        x < self.len.0 && y < self.len.1
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Entity> {
        if self.bounds(x, y) {
            self.list[x + (y * self.len.0)]
        }
        else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, element: Option<Entity>) {
        if self.bounds(x, y) {
            self.list[x + (y * self.len.0)] = element;
        }
    }
}

#[derive(Clone, Debug)]
//pub struct Direction(pub i8, pub i8);
pub struct Direction {
    pub direction: (i8, i8),
    pub previous: Option<(i8, i8)>,
}
impl Default for Direction {
    fn default() -> Self {
        Direction {
            direction: (1, 0),
            previous: None,
        }
    }
}
impl Component for Direction {
    type Storage = DenseVecStorage<Self>;
}

pub struct Snake {
    // Which direction the snake will go next tick.
    pub queued_direction: (i8, i8),

    // Length of the snake.
    pub length: u16,

    // End of the snake (end of tail).
    pub end: Option<Entity>,
}
impl Component for Snake {
    type Storage = DenseVecStorage<Self>;
}

// Flags that the snake should be extended
pub struct Extension(pub u16);
impl Component for Extension {
    type Storage = DenseVecStorage<Self>;
}

// Which kind of entity the tail inherits from.
pub struct Parent(pub Entity);
impl Component for Parent {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Default)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
}
impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

pub enum Type {
    Kill,
    Eat
}
impl Default for Type {
    fn default() -> Self {
        Type::Kill
    }
}
impl Component for Type {
    type Storage = NullStorage<Self>;
}

