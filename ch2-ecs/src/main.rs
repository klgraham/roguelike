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
#[derive(Component)]
struct LeftMover {}

#[derive(Component, Debug)]
struct Player {}

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

    for (_player, pos) in (&mut players, &mut positions).join() {
        // check to make sure we're in bounds
        pos.x = min(79, max(0, pos.x + dx));
        pos.y = min(49, max(0, pos.y + dy));
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
        }
    }
}



impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // clear the screen. Generally do at the beginning of a frame
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

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

/*
We'll make a system that lets the LeftMover move left.
Systems are ways to bundle logic for entities and components.
*/
struct LeftWalker {} // empty struct that gives us somewhere to put the logic

/* 
Spec's System trait
The lifetime here is saying that the components must last long enough for 
the system to run. So, this is the LeftWalker system. It works with entities
that have LeftMover and Position components.
*/
impl<'a> System<'a> for LeftWalker {
    // Tells Specs what data the system needs: 
    // read access for LeftMover components, write access for Position components
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        // gives us entities with both LeftMover and Position
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            // screen wrap
            if pos.x < 0 { pos.x = 79;}
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker{};
        lw.run_now(&self.ecs);
        // tells Specs that if any changes were queued up, they should be applied now
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
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    // Now we can create entities with positions that can also be drawn on the screen
    gs.ecs
        .create_entity()
        .with(Position::new(40, 25))
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position::new(i * 7, 20))
            .with(Renderable {
                // to_cp437 is a helper RLTK provides to let you type/paste Unicode and get the equivalent member of the old DOS/CP437 character set.
                // see here for more: https://dwarffortresswiki.org/index.php/Character_table
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover{})
            .build();
    }

    rltk::main_loop(context, gs);
}
