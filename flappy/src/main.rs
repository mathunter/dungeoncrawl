mod game_mode;
mod obstacle;
mod player;
mod state;

use bracket_lib::prelude::*;

use state::State;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const FRAME_DURATION: f32 = 75.0;

fn main() -> BError {
    // Create a new bracket context
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    // Run the main loop
    main_loop(context, State::new())
}
