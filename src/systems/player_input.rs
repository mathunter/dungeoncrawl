use crate::prelude::*;

// A system that handles player input
#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    // Capture the input key
    if let Some(key) = *key {
        // Map the key into a point delta
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };

        // Get the player
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        // Get the enemies
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

        // If we have some direction of movement, add a command to either move or attack
        let mut did_something = false;
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;

            // Iterate the enemies. If we hit one, issue an attack command
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    did_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            // If we didn't hit an enemy, issue a move command
            if !hit_something {
                did_something = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }

        // If we didn't do anything, restore some player health
        if !did_something {
            if let Ok(mut health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                println!("Here");
                health.current = i32::min(health.max, health.current + 1);
            }
        }

        // Flip to the next state
        *turn_state = TurnState::PlayerTurn;
    }
}
