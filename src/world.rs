use crate::{organism, food, tile, position};
extern crate rand;
use rand::thread_rng;
use rand::Rng;

pub struct World {
    pub organisms: Vec<organism::Organism>,
    pub food: food::Food,
    pub day: u32,
    pub grid: [[tile::Tile; 256]; 256] // 256x256 Grid
}

impl World {
    pub fn new(food: food::Food) -> World {
        World {
            organisms: Vec::new(),
            food,
            day: 0,
            grid: [[tile::Tile::None; 256]; 256] 
        }
    }

    pub fn add_organism(&mut self, organism: organism::Organism) {
        self.organisms.push(organism);
    }

    pub fn spawn_food(&mut self) {
        let mut rng = thread_rng();
        for _ in 0..self.food.count {
            'm :loop {
                let x = rng.gen_range(0..256);
                let y = rng.gen_range(0..256);
                match self.grid[x][y] {
                    tile::Tile::None => {
                        self.grid[x][y] = tile::Tile::Food(self.food);
                        
                        break 'm;
                    },
                    _ => continue 'm
                };
            }
        }
    }

    pub fn spawn_organism(&mut self) {
        let mut rng = thread_rng();
        for (i, org) in self.organisms.clone().into_iter().enumerate() {
            'm :loop {
                let x = rng.gen_range(0..256);
                let y = rng.gen_range(0..256);
                match self.grid[x][y] {
                    tile::Tile::None => {
                        self.grid[x][y] = tile::Tile::Organism(org);
                        self.organisms[i].position = position::Position::new(x, y);
                        break 'm;
                    },
                    _ => continue 'm
                };
            }
        }
    }

    pub fn clear_grid(&mut self) {
        self.grid = [[tile::Tile::None; 256]; 256];
    } 

    pub fn increment_day(&mut self) {
        self.day += 1;
    }
}
