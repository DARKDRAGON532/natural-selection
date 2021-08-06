use crate::{food, organism};

#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum Tile {
    Food(food::Food),
    Organism(organism::Organism),
    None
}