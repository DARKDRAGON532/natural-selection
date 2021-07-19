extern crate rand;

use rand::thread_rng;
use rand::Rng;

#[derive(Clone, Copy)]
struct Position {
    x: u16,
    z: u16
}

impl Position {
    fn new(x: u16, z: u16) -> Position {
        Position { x, z } 
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Food(Food),
    Organism(Organism),
    None
}

#[derive(Clone, Copy)]
struct Food {
    count: u64
}

impl Food {
    fn new(count: u64) -> Food {
        Food { count }
    }

    fn add(&mut self, count: u64) {
        self.count += count;
    }

    fn reduce(&mut self, count: u64) {
        self.count -= count;
    }
}

#[derive(Clone, Copy)]
struct Traits {
    speed: u8,
    size: u8,
    sense_radius: u8
}

impl Traits {
    fn new(speed: u8, size: u8, sense_radius: u8) -> Traits {
        Traits { speed, size, sense_radius }
    }
}

#[derive(Clone, Copy)]
struct Organism {
    traits: Traits,
    food_eaten: u16,
    energy: u16,
    position: Position,
}

impl Organism {
    fn new(traits: Traits) -> Organism {
        let mut rng = thread_rng();
        Organism { traits, food_eaten: 0, energy: 0, position: Position::new(rng.gen_range(0..512), rng.gen_range(0..512)) }
    }

    fn reproduce(&mut self) -> Organism {
        let mut rng = thread_rng();
        let mut speed = self.traits.speed;
        let mut size = self.traits.size;
        let mut sense_radius = self.traits.sense_radius;

        // mutation for speed
        if rng.gen_range(0..1000) == 0 {
            if rng.gen_range(0..2) == 0 {
                speed += 1;
            } 
            else {
                speed -= 1;
            }
        };

        // mutation for size
        if rng.gen_range(0..1000) == 0 {
            if rng.gen_range(0..2) == 0 {
                size += 1;
            } 
            else {
                size -= 1;
            }
        };

        // mutation for sense_radius
        if rng.gen_range(0..1000) == 0 {
            if rng.gen_range(0..2) == 0 {
                sense_radius += 1;
            } 
            else {
                sense_radius -= 1;
            }
        };

        Organism::new(Traits::new(speed, size, sense_radius))
        
    }
}

struct World {
    organisms: Vec<Organism>,
    food: Food,
    day: u32,
    grid: [[Tile; 512]; 512] // 512x512 Grid
}

impl World {
    fn new(food: Food) -> World {
        World {
            organisms: Vec::new(),
            food,
            day: 0,
            grid: [[Tile::None; 512]; 512]   
        }
    }

    fn add_organism(&mut self, organism: Organism) {
        self.organisms.push(organism);
    }

    fn spawn_food(&self) {
        for _ in 0..self.food.count {
            let mut rng = thread_rng();
        }
    }
}
