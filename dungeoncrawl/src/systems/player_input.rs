use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };

        if delta.x != 0 || delta.y != 0 {
            let mut player_points = <&mut Point>::query().filter(component::<Player>());
            player_points.iter_mut(ecs).for_each(|player_point| {
                let destination = *player_point + delta;
                if map.can_enter_tile(destination) {
                    *player_point = destination;
                    camera.on_player_move(destination);
                }
            })
        }
    }
}
