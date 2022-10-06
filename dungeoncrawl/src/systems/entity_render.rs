use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    // Create a new draw batch
    let mut draw_batch = DrawBatch::new();

    // Target the foreground console
    draw_batch.target(1);

    // Query for all renderable entities (having Point and Render facets) and render them
    let offset = Point::new(camera.left_x, camera.top_y);
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    // Submit the batch to be rendered, well after the background
    draw_batch.submit(5000).expect("Batch error");
}
