use specs::{Component, Entity, DenseVecStorage};
use amethyst::ecs::resources::Button;

#[derive(Debug, Default)]
pub struct Tick {
    // How many ticks have happened so far
    pub ticks: u64,

    // Did a tick occur this frame.
    pub ticked: bool,

    pub game_over: bool,
}

// Grid of snake parts.
pub struct Grid {
    list: Vec<Option<Entity>>,
    pub len: (usize, usize),
}
impl Grid {
    pub fn new(x: usize, y: usize) -> Self {
        let max = x + y * x;
        let mut list = Vec::with_capacity(max);

        for _ in 0..max {
            list.push(None);
        }
        
        Grid {
            list: list,
            len: (x, y),
        }
    }

    fn bounds(&self, x: usize, y: usize) -> bool {
        x < self.len.0 && y < self.len.1
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x + (y * self.len.0)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Entity> {
        if self.bounds(x, y) {
            self.list[self.index(x, y)]
        }
        else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, element: Option<Entity>) {
        if self.bounds(x, y) {
            let index = self.index(x, y);
            self.list[index] = element;
        }
    }

    pub fn print(&self) {
        for x in 0..self.len.0 {
            for y in 0..self.len.1 {
                match self.get(x, y) {
                    Some(_) => print!("1 "),
                    None => print!("0 "),
                }
            }
            println!();
        }
    }
}

#[derive(Clone, Debug)]
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

// Controls for each snake.
pub struct Controls {
    pub left: Button,
    pub right: Button,
    pub up: Button,
    pub down: Button,
}
impl Component for Controls {
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

#[derive(Debug)]
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
    type Storage = DenseVecStorage<Self>;
}

