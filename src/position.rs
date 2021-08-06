#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y } 
    }

    pub fn get_position(&self) -> (usize, usize){
        (self.x, self.y)
    }
}