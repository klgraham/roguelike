use rltk::{Console, Rltk, RGB, RandomNumberGenerator};
use std::cmp::{max, min};

/// Types of tiles
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor
}

// Takes position (x, y) and returns a vector index
// TODO: Replace this and the map with a Map struct that has subscript
// TODO: make this generic in terms of Map dimensions
//  will give a reusable Map struct
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn new_map() -> Vec<TileType> {
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