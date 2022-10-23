use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] viewport: &mut Viewport,
    #[resource] turn_state: &mut TurnState
) {
    // Capture the input key
    if let Some(key) = key {
        // Map the key into a point delta
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };

        // If we have some direction of movement, move the player
        if delta.x != 0 || delta.y != 0 {
            let mut player_points = <&mut Point>::query().filter(component::<Player>());
            player_points.iter_mut(ecs).for_each(|player_point| {
                let destination = *player_point + delta;
                if map.can_enter_tile(destination) {
                    *player_point = destination;
                    viewport.on_player_move(destination);
                    *turn_state = TurnState::PlayerTurn;
                }
            })
        }
    }
}
