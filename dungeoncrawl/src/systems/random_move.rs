use crate::prelude::*;

// A system that handles the random movement of entities so annotated
#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // Find our random movers
    let mut random_movers = <(Entity, &Point, &MovingRandomly)>::query();

    // Find our living (health-bearing) entities
    let mut living_entities = <(Entity, &Point, &Health)>::query();

    // For each mover, generate a random direction, and move the mover if we can
    random_movers.iter(ecs).for_each(|(entity, pos, _)| {
        // Generate a new destination
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        // Determine if any of our living entities is attacking a player
        let mut attacking = false;
        living_entities
            .iter(ecs)
            .filter(|(_, target_pos, _)| **target_pos == destination)
            .for_each(|(victim, _, _)| {
                // If the victim is the player, push an attack message
                if ecs
                    .entry_ref(*victim)
                    .unwrap()
                    .get_component::<Player>()
                    .is_ok()
                {
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: *entity,
                            victim: *victim,
                        },
                    ));
                }
                attacking = true;
            });

        // If we didn't attack, push a move message
        if !attacking {
            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        }
    });
}
