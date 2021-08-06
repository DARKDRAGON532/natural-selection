use natural_selection::*;

fn main() {
    let food = food::Food::new(100);
    let mut world = world::World::new(food);
    world.spawn_food();
    world.add_organism(organism::Organism::new(traits::Traits::new(1,1,1)));
    world.spawn_organism();
    println!("{:?}", world.grid);

}