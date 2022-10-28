use crate::prelude::*;

// A system that handles state change between turns
#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    // Figure out the next turn state
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => turn_state.clone(),
    };

    // Find the player health
    let mut player_healths = <&Health>::query().filter(component::<Player>());
    let player_health = player_healths.iter(ecs).next().unwrap();
    if player_health.current < 1 {
        new_state = TurnState::GameOver;
    }

    // Update the turn state
    *turn_state = new_state;
}
