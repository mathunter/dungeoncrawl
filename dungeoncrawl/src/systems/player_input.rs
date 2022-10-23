use crate::prelude::*;

// A system that handles player input
#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    // Get the player
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

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

        // If we have some direction of movement, add a command to move the player
        if delta.x != 0 || delta.y != 0 {
            players.iter(ecs).for_each(|(entity, pos)| {
                let destination = *pos + delta;
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            });
            *turn_state = TurnState::PlayerTurn;
        }
    }
}
