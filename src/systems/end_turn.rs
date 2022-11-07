use crate::prelude::*;

// A system that handles state change between turns
#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    // Get the Amulet position
    let mut amulet_query = <&Point>::query().filter(component::<AmuletOfYala>());
    let amulet_pos = amulet_query.iter(ecs).next().unwrap();

    // Figure out the next turn state
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => *turn_state,
    };

    // Iterate over player health and position, and select the game state appropriately
    let mut player_query = <(&Health, &Point)>::query().filter(component::<Player>());
    player_query
        .iter(ecs)
        .for_each(|(player_health, player_pos)| {
            // If the player health falls below 1, it's game over
            if player_health.current < 1 {
                new_state = TurnState::GameOver;
            }

            // If the player intersects the amulet, it's victory
            if player_pos == amulet_pos {
                new_state = TurnState::Victory;
            }
        });

    // Update the turn state
    *turn_state = new_state;
}
