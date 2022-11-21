use crate::map_builder::automata::CellularAutomataArchitect;
use crate::map_builder::drunkard::DrunkardsWalkArchitect;
use crate::map_builder::prefab::apply_prefab;
use crate::map_builder::rooms::RoomsArchitect;
use crate::prelude::*;
use empty::EmptyArchitect;
use themes::*;

mod automata;
mod drunkard;
mod empty;
mod prefab;
mod rooms;
mod themes;

const NUM_ROOMS: usize = 20;

///
/// A struct that defines the information required to build out a game map
///
pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
    pub theme: Box<dyn MapTheme>,
}

///
/// A trait that defines a mechanism that architects maps according to an algorithm
///
trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

///
/// A trait that defines a map theme
///
pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
}

///
/// An implementation of the MapBuilder trait
///
impl MapBuilder {
    ///
    /// Fills the map with the specified TileType
    /// * `tile_type` - the TileType of the tile to fill
    fn fill(&mut self, tile_type: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile_type);
    }

    ///
    /// Finds the most distant point on the map from the specified point
    /// * `source_point` - the point from which to find the most distant point
    fn find_most_distant(&self, source_point: Point) -> Point {
        // Using a Dijkstra map, find the index that is furthest from the player, and map that to a point
        let search_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &[self.map.point2d_to_index(source_point)],
            &self.map,
            1024.0,
        );
        const UNREACHABLE: &f32 = &f32::MAX;
        let furthest_index = search_map
            .map
            .iter()
            .enumerate()
            .filter(|(_, dist)| *dist < UNREACHABLE)
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        self.map.index_to_point2d(furthest_index)
    }

    ///
    /// Builds random rooms
    /// * `rng` - a RandomNumberGenerator
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        // Generate rooms up to our configured maximum
        while self.rooms.len() < NUM_ROOMS {
            // Create a room with random dimensions
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            // Check to see if the new room overlaps with an existing room
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            // If the new room does not overlap an existing room, fill render the room on the map
            //  as floor tiles
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room);
            }
        }
    }

    ///
    /// Builds corridors between rooms
    /// * `rng` - a RandomNumberGenerator
    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        // Clone the rooms for corridor builder
        let mut rooms = self.rooms.clone();

        // Sort the rooms by their center X coordinates
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        // Iterate through the rooms, skipping every other, and build tunnels between them
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }

    ///
    /// Builds a vertical tunnel between two points
    /// * `y1` - the starting y coordinate
    /// * `y2` - the ending y coordinate
    /// * `x` - the shared x coordinate
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    ///
    /// Builds a horizontal tunnel between two points
    /// * `x1` - the starting x coordinate
    /// * `x2` - the ending x coordinate
    /// * `y` - the shared y coordinate
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    ///
    /// Creates a new instance
    /// * `rng` - a RandomNumberGenerator
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        //
        // Randomly select the architect
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => Box::new(DrunkardsWalkArchitect {}),
            1 => Box::new(RoomsArchitect {}),
            _ => Box::new(CellularAutomataArchitect {}),
        };

        // Use the architect to build the map
        let mut mb = architect.new(rng);

        // Apply a prefab fortress
        apply_prefab(&mut mb, rng);

        // Randomly select the theme for the map
        mb.theme = match rng.range(0, 2) {
            0 => DungeonTheme::new(),
            _ => ForestTheme::new(),
        };

        mb
    }

    ///
    /// Spawns monsters from the specified start point
    /// * `start` - the start point
    /// * `rng` - a RandomNumberGenerator
    fn spawn_monsters(&self, start_point: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_MONSTERS: usize = 50;

        // Create the collection of tiles on which we can spawn monsters
        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Floor
                    && DistanceAlg::Pythagoras
                        .distance2d(*start_point, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        // Create a new collection of monster spawn points
        let mut monster_spawns = Vec::new();
        for _ in 0..NUM_MONSTERS {
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            monster_spawns.push(spawnable_tiles[target_index]);
            spawnable_tiles.remove(target_index);
        }
        monster_spawns
    }
}
