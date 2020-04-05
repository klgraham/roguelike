use rltk::{Console, GameState, Rltk, RGB};
use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;

/*
Specs requires you to register the components at launch. They get registered
in the world state. Here, the World is an entity-component system (ECS).
*/
pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
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
