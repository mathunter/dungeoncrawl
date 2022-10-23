use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    // Find our movers
    let mut movers = <(&mut Point, &MovingRandomly)>::query();

    // For each mover, generate a random direction, and move the mover if we can
    movers.iter_mut(ecs).for_each(|(pos, _)| {
        // Generate a new destination
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        // If we can move into the new destination, do so
        if map.can_enter_tile(destination) {
            *pos = destination;
        }
    });
}
