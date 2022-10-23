use crate::prelude::*;

// A system that handles rendering of entities
#[system]
#[read_component(Point)]
#[write_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] viewport: &Viewport) {
    // Create a new draw batch
    let mut draw_batch = DrawBatch::new();

    // Target the foreground console
    draw_batch.target(1);

    // Calculate the offset, being the top left of the viewport
    let offset = Point::new(viewport.left_x, viewport.top_y);

    // Query for all renderable entities (having Point and Render facets) and render them
    let mut renderables = <(&Point, &Render)>::query();
    renderables.iter(ecs).for_each(|(pos, render)| {
        draw_batch.set(*pos - offset, render.color, render.glyph);
    });

    // Submit the batch to be rendered, well after the background
    draw_batch.submit(5000).expect("Batch error");
}
