use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use std::cmp::{max, min};

// This means the next crate listed will contain macro code
#[macro_use]
extern crate specs_derive;
// "extern crate" must be called when the crate has macros and you want them

/// Entity Position, allows drawing to screen
#[derive(Component)] // makes `Position` a Specs component
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

/// A Component for drawing things to the screen
#[derive(Component)]
struct Renderable {
    glyph: u8,
    fg: RGB, // foreground color?
    bg: RGB, // background color?
}

// Components with no data are called "tag" components.
#[derive(Component, Debug)]
struct Player {}

/// Types of tiles
// The Copy derive macro makes this copy on assignment instead of move
// Clone adds a .clone() method
// PartialEq gives us ==
#[derive(PartialEq, Copy, Clone)]
enum TileType {
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

fn new_map() -> Vec<TileType> {
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

fn draw_map(map: &[TileType], ctx: &mut Rltk) {
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

/*
Specs requires you to register the components at launch. They get registered
in the world state. Here, the World is an entity-component system (ECS).
*/
struct State {
    ecs: World,
}

fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination = xy_idx(pos.x + dx, pos.y + dy);

        // Can't walk through walls
        if map[destination] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + dx));
            pos.y = min(49, max(0, pos.y + dy));
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {} // nothing happens
        Some(key) => match key {
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            // VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            // VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            // VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            // VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {} // do nothing if any other key is hit
        },
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // clear the screen. Generally do at the beginning of a frame
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        // asks the ECS for read access to where the Position components are stored
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        // The join call only returns entities that have both
        for (pos, render) in (&positions, &renderables).join() {
            // sets a single terminal character to specific glyphs/colors
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

fn main() {
    use rltk::RltkBuilder;
    // make a terminal that 80 characters wide x 50 characters high
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial") // title of the window
        .build();

    // create a new World (game state)
    let mut gs = State { ecs: World::new() };

    // tell the ECS about the components we made
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs.insert(new_map());

    // Now we can create entities with positions that can also be drawn on the screen
    gs.ecs
        .create_entity()
        .with(Position::new(40, 25))
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    rltk::main_loop(context, gs);
}
