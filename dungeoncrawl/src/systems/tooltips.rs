use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] viewport: &Viewport) {
    // Set the render position
    let offset = Point::new(viewport.left_x, viewport.top_y);
    let map_pos = *mouse_pos + offset;

    // Create a new draw batch
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    // Get the positions and render them
    let mut positions = <(Entity, &Point, &Name)>::query();
    positions.iter(ecs).filter(|(_, pos, _)| **pos == map_pos).for_each(|(entity, _, name)| {
        let screen_pos = *mouse_pos * 4;
        let display = if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
            format!("{} : {} hp", &name.0, health.current)
        } else {
            name.0.clone()
        };
        draw_batch.print(screen_pos, &display);
    });
    draw_batch.submit(10100).expect("Batch error");
}
