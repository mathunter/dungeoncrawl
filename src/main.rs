mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;
mod viewport;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use crate::viewport::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
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

        // Add the map and viewport to the resources
        let mut resources = Resources::default();
        resources.insert(map_builder.map);
        resources.insert(Viewport::new(map_builder.player_start));

        // Add the initial awaiting turn state to the resources
        resources.insert(TurnState::AwaitingInput);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
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

        // Clear the HUD console
        ctx.set_active_console(2);
        ctx.cls();

        // Add any pressed key into the resources
        self.resources.insert(ctx.key);

        // Render the mouse coordinates
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        // Execute the appropriate system, depending on the current turn state
        let current_state = *self.resources.get::<TurnState>().unwrap();
        match current_state {
            TurnState::AwaitingInput => {
                self.input_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::PlayerTurn => {
                self.player_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::MonsterTurn => {
                self.monster_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
        }

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
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    // Run the main loop
    main_loop(context, State::new())
}
