#[derive(Clone, Copy)]
#[derive(Debug)]
/*
    One Meter = 1 Tile
    Max Speed = x Tile / Second
    x = self.spped * 5
    speed can be between 1 and self.speed * 5
    For every 5 meters the organism moves, it loses 1 energy
    

    Size has no effect on how many tiles the organism takes.
    Your size consumes more energy.
    Size lets you consume smaller organisms.

    Sense radius is the radius of how far the organism can sense food.
    If it senses food it will go towards the food or else it moves in random directions.
*/
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