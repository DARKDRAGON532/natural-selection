extern crate rand;

use rand::thread_rng;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub z: u16
}

impl Position {
    pub fn new(x: u16, z: u16) -> Position {
        Position { x, z } 
    }
}

#[derive(Clone, Copy)]
pub enum Tile<'a> {
    Food(&'a Food),
    Organism(&'a Organism),
    None
}

#[derive(Clone, Copy)]
pub struct Food {
    pub count: u64
}

impl Food {
    pub fn new(count: u64) -> Food {
        Food { count }
    }

    pub fn add(&mut self, count: u64) {
        self.count += count;
    }

    pub fn reduce(&mut self, count: u64) {
        self.count -= count;
    }
}

#[derive(Clone, Copy)]
pub struct Traits {
    pub speed: u8,
    pub size: u8,
    pub sense_radius: u8
}

impl Traits {
    pub fn new(speed: u8, size: u8, sense_radius: u8) -> Traits {
        Traits { speed, size, sense_radius }
    }
}

#[derive(Clone, Copy)]
pub struct Organism {
    pub traits: Traits,
    pub food_eaten: u16,
    pub energy: u16,
    pub position: Position,
}

impl Organism {
    pub fn new(traits: Traits) -> Organism {
        let mut rng = thread_rng();
        Organism { traits, food_eaten: 0, energy: 0, position: Position::new(rng.gen_range(0..512), rng.gen_range(0..512)) }
    }

    pub fn reproduce(&mut self) -> Organism {
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

pub struct World<'a> {
    pub organisms: Vec<Organism>,
    pub food: Food,
    pub day: u32,
    pub grid: [[Tile<'a>; 512]; 512] // 512x512 Grid
}

impl World<'_> {
    pub fn new<'a>(food: Food) -> World<'a> {
        World {
            organisms: Vec::new(),
            food,
            day: 0,
            grid: [[Tile::None; 512]; 512]   
        }
    }

    pub fn add_organism(&mut self, organism: Organism) {
        self.organisms.push(organism);
    }

    pub fn spawn_food(&self) {
        let mut rng = thread_rng();
        for _ in 0..self.food.count {
            let x = rng.gen_range(0..512);
            let z = rng.gen_range(0..512);
            match self.grid[x][z] {
                Tile::None => self.grid[x][z] = Tile::Food(&self.food),
                _ => println!("error fix me")
            };
        }
    }
}
