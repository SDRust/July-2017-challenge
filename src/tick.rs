
use amethyst::ecs::resources::Time;
use specs::{Entities, Join, Fetch, FetchMut, System};

use components::Tick;

pub const TICK_RATE: u64 = 100_000_000 ; // 1/10th of a second
pub const TICKS_PER_SECOND: u64 = 1_000_000_000 / TICK_RATE;
pub const FPS_SAMPLE: usize = 20;

// Deals with game ticks
pub struct TickSystem {
    accumulator: u64,

    current: usize,
    average_fps: Vec<u64>,
}
impl Default for TickSystem {
    fn default() -> Self {
        let mut samples = Vec::new();
        for _ in 0..FPS_SAMPLE {
            samples.push(0);
        }

        TickSystem {
            accumulator: 0,
            current: 0,
            average_fps: samples,
        }
    }
}
impl<'a> System<'a> for TickSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, Time>,
        FetchMut<'a, Tick>,
    );
    fn run(&mut self, (entities, time, mut tick): Self::SystemData) {
        if time.delta_time.subsec_nanos() > TICK_RATE as u32 {
            self.accumulator = TICK_RATE;
        }
        else {
            self.accumulator += time.delta_time.subsec_nanos() as u64;
        }
        
        self.average_fps[self.current] = time.delta_time.subsec_nanos() as u64 / 1_000u64;
        self.current += 1;
        if self.current >= FPS_SAMPLE {
            self.current = 0;
        }

        if self.accumulator >= TICK_RATE {
            self.accumulator = 0;
            tick.ticks += 1;
            tick.ticked = true;

            if tick.ticks % (TICKS_PER_SECOND * 3) == 0 { // print out every 3 seconds
                let samples = self.average_fps.iter().sum::<u64>() / FPS_SAMPLE as u64;
                println!("FPS: {:.2}", 1_000_000f64 / samples as f64);
                //println!("ENTITIES: {}", (&*entities).join().count());
            }
        }
        else {
            tick.ticked = false;
        }
    }
}
