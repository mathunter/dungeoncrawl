use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    // Get the player health
    let mut player_healths = <&Health>::query().filter(component::<Player>());
    let player_health = player_healths.iter(ecs).next().unwrap();

    // Draw the health bar
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );

    // Draw the instructions
    draw_batch.print_centered(1, "Explore the dungeon. Cursor keys to move.");

    draw_batch.submit(10000).expect("Batch error");
}
