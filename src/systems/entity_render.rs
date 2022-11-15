use crate::prelude::*;

// A system that handles rendering of entities
#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] viewport: &Viewport) {
    // Create a new draw batch
    let mut draw_batch = DrawBatch::new();

    // Target the foreground console
    draw_batch.target(1);

    // Calculate the offset, being the top left of the viewport
    let offset = Point::new(viewport.left_x, viewport.top_y);

    // Get the player FOV
    let mut player_fov_query = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = player_fov_query.iter(ecs).next().unwrap();

    // Query for all renderable entities (having Point and Render facets),
    // filter only those visible to the player, and render them
    let mut renderable_query = <(&Point, &Render)>::query();
    renderable_query
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    // Submit the batch to be rendered, well after the background
    draw_batch.submit(5000).expect("Batch error");
}
