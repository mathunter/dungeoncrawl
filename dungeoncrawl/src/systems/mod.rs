use crate::prelude::*;

mod collision;
mod entity_render;
mod map_render;
mod player_input;

pub fn build_scheduler() -> Schedule {
    // Add the systems to the scheduler
    Schedule::builder()
        .add_system(collision::collisions_system())
        .add_system(entity_render::entity_render_system())
        .add_system(map_render::map_render_system())
        .add_system(player_input::player_input_system())
        .build()
}
