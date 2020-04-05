use super::Rect;
use rltk::{Console, RandomNumberGenerator, Rltk, RGB};
use std::cmp::{max, min};

/// Types of tiles
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

// Takes position (x, y) and returns a vector index
// TODO: Replace this and the map with a Map struct that has subscript
// TODO: make this generic in terms of Map dimensions
//  will give a reusable Map struct
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

/// Makes a map with walls at the edges and 400 randomly placed walls.
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];

    // Make the boundaries walls
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // add some random walls
    let mut rng = rltk::RandomNumberGenerator::new();

    let num_walls = 400;
    for _ in 0..num_walls {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let i = xy_idx(x, y);

        if i != xy_idx(40, 25) {
            map[i] = TileType::Wall;
        }
    }

    map
}

fn add_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in (room.y1 + 1)..=room.y2 {
        for x in (room.x1 + 1)..=room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

pub fn new_map_rooms_and_corridors() -> Vec<TileType> {
    let mut map = vec![TileType::Wall; 80 * 50];
    let mut rooms: Vec<Rect> = Vec::new();

    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, 80 - w - 1) - 1;
        let y = rng.roll_dice(1, 50 - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;

        // Check if the new room overlaps with existing rooms.
        // If it does, skip it.
        for other_room in rooms.iter() {
            if new_room.overlaps_with(other_room) {
                ok = false
            }
        }

        if ok {
            add_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();

                if let Some(prev_room) = rooms.last() {
                    let (prev_x, prev_y) = prev_room.center();

                    // 50% chance of order to draw tunnels
                    if rng.range(0, 2) == 1 {
                        add_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                        add_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                    } else {
                        add_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                        add_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                    }
                }
            }

            rooms.push(new_room);
        }
    }

    map
}

fn add_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let i = xy_idx(x, y);
        if i > 0 && i < 80 * 50 {
            map[i as usize] = TileType::Floor;
        }
    }
}

fn add_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let i = xy_idx(x, y);
        if i > 0 && i < 80 * 50 {
            map[i as usize] = TileType::Floor;
        }
    }
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;

    for tile in map.iter() {
        // Render a tile depending on tile type
        match tile {
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
            }
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0., 1., 0.),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'),
                );
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
