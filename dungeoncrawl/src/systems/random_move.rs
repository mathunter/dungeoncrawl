use crate::prelude::*;

// A system that handles the random movement of entities so annotated
#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // Find our movers
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();

    // For each mover, generate a random direction, and move the mover if we can
    movers.iter_mut(ecs).for_each(|(entity, pos, _)| {
        // Generate a new destination
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        // Add a command to indicate the desire to move
        commands.push((
            (),
            WantsToMove {
                entity: *entity,
                destination,
            },
        ));
    });
}
