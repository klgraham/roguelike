// extern crate rltk;
use super::Rect;
use rltk::{Algorithm2D, BaseMap, Console, DistanceAlg, Point, RandomNumberGenerator, Rltk, RGB};
use specs::prelude::*;
use std::cmp::{max, min};
use std::ops::{Index, IndexMut};

/// Types of tiles
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

/// Map
pub struct Map {
    // vector of all tiles in the map
    pub tiles: Vec<TileType>,
    // the rooms in the map
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    // tiles that have been visited
    // once these are set to true, they will not change again
    pub revealed_tiles: Vec<bool>,
    // tiles that are currently visible
    // these can change back and forth
    pub visible_tiles: Vec<bool>,
    // keep track of which tiles are able to be occupied
    pub blocked: Vec<bool>,
    // track what is on each tile
    pub tile_content: Vec<Vec<Entity>>,
}

// These let me use subscripting
impl Index<(i32, i32)> for Map {
    type Output = TileType;

    #[inline]
    fn index(&self, (x, y): (i32, i32)) -> &TileType {
        assert!(self.contains(x, y), "Map position out of bounds.");
        let idx = (y as usize * self.width as usize) + x as usize;
        &self.tiles[idx]
    }
}

impl IndexMut<(i32, i32)> for Map {
    #[inline]
    fn index_mut(&mut self, (x, y): (i32, i32)) -> &mut TileType {
        assert!(self.contains(x, y), "Map position out of bounds.");
        let idx = (y as usize * self.width as usize) + x as usize;
        &mut self.tiles[idx]
    }
}

impl Map {
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x > 0 && x < self.width && y > 0 && y < self.height
    }

    pub fn idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    // Given a room, update the tiles to be floor and not walls
    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in (room.y1 + 1)..=room.y2 {
            for x in (room.x1 + 1)..=room.x2 {
                self[(x, y)] = TileType::Floor;
            }
        }
    }

    fn add_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            if self.contains(x, y) {
                self[(x, y)] = TileType::Floor;
            }
        }
    }

    fn add_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            if self.contains(x, y) {
                self[(x, y)] = TileType::Floor;
            }
        }
    }

    /// Sets each tile that is a wall tile to blocked
    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    /// Clears the index that tracks what is on each tile
    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    /// Returns true if (x,y) is an exit from the room
    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if !self.contains(x, y) {
            return false;
        }

        let i = self.idx(x, y);
        !self.blocked[i]
    }

    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
            revealed_tiles: vec![false; 80 * 50],
            visible_tiles: vec![false; 80 * 50],
            blocked: vec![false; 80 * 50],
            tile_content: vec![Vec::new(); 80 * 50],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;

            // Does new room overlap with existing rooms? If yes, skip it.
            for other_room in map.rooms.iter() {
                if new_room.overlaps_with(other_room) {
                    ok = false;
                }
            }

            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();

                    // 50% chance of order to draw tunnels
                    if rng.range(0, 2) == 1 {
                        map.add_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.add_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.add_vertical_tunnel(prev_y, new_y, prev_x);
                        map.add_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    // This suggests it's possible to have things that can temporarily modify the opacity of a wall!
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> Vec<(usize, f32)> {
        let mut exits: Vec<(i32, f32)> = Vec::new();
        let i = idx as i32;
        let x = i % self.width;
        let y = i / self.width;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((i - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((i + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((i - self.width, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((i + self.width, 1.0))
        };

        // Diagonals
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((i - self.width) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((i - self.width) + 1, 1.45));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((i + self.width) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((i + self.width) + 1, 1.45));
        }
        // this is a hack to address an issue in the tutorial code not matching BaseMap
        let exits2: Vec<(usize, f32)> = exits.iter().map(|(a, b)| (*a as usize, *b)).collect();
        exits2
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;

    for (i, tile) in map.tiles.iter().enumerate() {
        // Render a tile depending on tile type
        if map.revealed_tiles[i] {
            let glyph;
            let mut fg;

            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437('.');
                    fg = RGB::from_f32(0.0, 0.5, 0.5);
                }
                TileType::Wall => {
                    glyph = rltk::to_cp437('#');
                    fg = RGB::from_f32(0., 1., 0.);
                }
            }

            if !map.visible_tiles[i] {
                fg = fg.to_greyscale()
            }
            ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
