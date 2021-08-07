extern crate rand;
use rand::thread_rng;
use rand::Rng;
use crate::{position, traits, world, tile};

/*
    If the organism consumes 1 food it lives till the next day
    If the organism consumes another piece of food it reproduces the next day.
    Each food eaten after the two inital food will be stored as energy
    1 extra food = 5 energy
    Extra energy is added the next day.

    All organisms get incremented 50 energy + extra energy everyday

*/

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Organism {
    pub traits: traits::Traits,
    pub food_eaten: u16,
    pub energy: u16,
    pub tiles_moved: u16,
    pub position: position::Position
}

impl Organism {
    pub fn new(traits: traits::Traits) -> Organism {
        let mut rng = thread_rng();
        Organism { traits, food_eaten: 0, energy: 0, tiles_moved: 0, position: position::Position::new(rng.gen_range(0..256), rng.gen_range(0..256)) }
    }

    pub fn reproduce(&mut self) -> Organism {
        let mut rng = thread_rng();
        let mut speed = self.traits.speed;
        let mut size = self.traits.size;
        let mut sense_radius = self.traits.sense_radius;

        for t in [&mut speed, &mut size, &mut sense_radius] {
            if rng.gen_range(0..1000) == 0 {
                if rng.gen_range(0..2) == 0 {
                    *t += 1;
                } 
                else {
                    if *t > 1 {
                        *t -= 1;
                    }
                    else {
                        *t += 1;
                    }
                }
            };
        }

        Organism::new(traits::Traits::new(speed, size, sense_radius))
        
    }

    pub fn movement(&mut self, world: &mut world::World) {
        let mut rng = thread_rng();
        let (old_x, old_y) = self.position.get_position();
        let direction = rng.gen_range(0..8);


        
        match direction {
            0 => {
                self.position.x += self.traits.speed as usize * 5;
            },
            1 => {
                self.position.y += self.traits.speed as usize * 5;
            },
            2 => {
                self.position.x -= self.traits.speed as usize * 5;
            }
            3 => {
                self.position.y -= self.traits.speed as usize * 5;
            },
            4 => {
                self.position.x += self.traits.speed as usize * 5;
                self.position.y += self.traits.speed as usize * 5;
            },
            5 => {
                self.position.x += self.traits.speed as usize * 5;
                self.position.y -= self.traits.speed as usize * 5;
            },
            6 => {
                self.position.x -= self.traits.speed as usize * 5;
                self.position.y -= self.traits.speed as usize * 5;
            },
            7 => {
                self.position.x -= self.traits.speed as usize * 5;
                self.position.y += self.traits.speed as usize * 5;
            },
            _ => () 
        }
        if let tile::Tile::Food(_) = world.grid[self.position.x][self.position.y] {
            self.food_eaten += 1
        }
        world.grid[old_x][old_y] = tile::Tile::None;
        world.grid[self.position.x][self.position.y] = tile::Tile::Organism(*self);
    }
}