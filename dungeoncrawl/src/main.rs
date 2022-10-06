mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::systems::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();

        // Build a new map
        let map_builder = MapBuilder::new(&mut rng);

        // Create a new ECS instance, into which we'll be sticking entities
        let mut ecs = World::default();

        // Spawn the player
        spawn_player(&mut ecs, map_builder.player_start);

        // Spawn monsters, one in each room, except for the first (where the player spawns)
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));

        // Add the map and camera to the resources
        let mut resources = Resources::default();
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Clear the background console
        ctx.set_active_console(0);
        ctx.cls();

        // Clear the foreground console
        ctx.set_active_console(1);
        ctx.cls();

        // Add any pressed key into the resources
        self.resources.insert(ctx.key);

        // Execute the systems
        self.systems.execute(&mut self.ecs, &mut self.resources);

        // Render all draw operations
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    // Create a new terminal context
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    // Run the main loop
    main_loop(context, State::new())
}