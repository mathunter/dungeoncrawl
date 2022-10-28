use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    // Find the player position
    let mut players = <(&Point, &Player)>::query();
    let player_pos = players.iter(ecs).next().unwrap().0;

    // Create a Dijkstra map to find the path to the player
    let player_idx = map_idx(player_pos.x, player_pos.y);
    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    // Find the chasers and move them
    let mut chasers = <(Entity, &Point, &ChasingPlayer)>::query();
    let mut entities = <(Entity, &Point, &Health)>::query();
    chasers.iter(ecs).for_each(|(chaser, chaser_pos, _)| {
        // Get the next destination for the chaser, as the exit from the current tile with the lowest
        // cost to move to the player's position. If there is one, move the chaser
        let chaser_idx = map_idx(chaser_pos.x, chaser_pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, chaser_idx, map) {
            // Get the distance to the player
            // If the player is more than 1.2 tiles away, use the destination. Else, use the player position.
            let distance = DistanceAlg::Pythagoras.distance2d(*chaser_pos, *player_pos);
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *player_pos
            };

            // See if the destination matches some other entity position, and resolve
            let mut did_attack = false;
            entities
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == destination)
                .for_each(|(victim, _, _)| {
                    // If the entity is the player, attack
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *chaser,
                                victim: *victim,
                            },
                        ));
                    }
                    did_attack = true;
                });

            // If we didn't attack, move instead
            if !did_attack {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *chaser,
                        destination,
                    },
                ));
            }
        }
    })
}
