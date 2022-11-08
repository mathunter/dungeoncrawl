use crate::prelude::*;

// A system that handles the rendering of the map
#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(ecs: &SubWorld, #[resource] map: &Map, #[resource] viewport: &Viewport) {
    // Get the player's FOV
    let mut fov_query = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov_query.iter(ecs).next().unwrap();

    // Start a new draw batch
    let mut draw_batch = DrawBatch::new();

    // Set the background color as the target
    draw_batch.target(0);

    // Iterate through the viewport dimensions and draw the map
    for y in viewport.top_y..viewport.bottom_y {
        for x in viewport.left_x..viewport.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(viewport.left_x, viewport.top_y);
            if map.in_bounds(pt) && player_fov.visible_tiles.contains(&pt) {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                draw_batch.set(pt - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }

    // Submit the batch to be drawn first
    draw_batch.submit(0).expect("Batch error");
}
