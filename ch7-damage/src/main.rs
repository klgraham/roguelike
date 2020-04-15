// extern crate rltk;
use rltk::{Console, GameState, Point, Rltk, RGB};
// extern crate specs;
use specs::prelude::*;

mod components;
use components::*;
mod rect;
use rect::Rect;
mod map;
use map::*;
mod player;
use player::*;
mod systems;
use systems::damage_system::{delete_the_dead, DamageSystem};
use systems::map_indexing_system::MapIndexingSystem;
use systems::melee_combat_system::MeleeCombatSystem;
use systems::monster_ai_system::MonsterAI;
use systems::visibility_system::VisibilitySystem;

// Allows us to "pause" the game
#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

/*
Specs requires you to register the components at launch. They get registered
in the world state. Here, the World is an entity-component system (ECS).
*/
pub struct State {
    pub ecs: World,
    pub run_state: RunState,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);

        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);

        let mut map_index = MapIndexingSystem {};
        map_index.run_now(&self.ecs);

        let mut melee = MeleeCombatSystem {};
        melee.run_now(&self.ecs);

        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    /*
    The things in here run every frame. The call to run_systems runs the systems
    that define the behavior of the entities.
     */
    fn tick(&mut self, ctx: &mut Rltk) {
        // clear the screen. Generally do at the beginning of a frame
        ctx.cls();

        // Only the run the simulation when the game isn't paused
        if self.run_state == RunState::Running {
            self.run_systems();
            // pause after each step
            self.run_state = RunState::Paused;
        } else {
            self.run_state = player_input(self, ctx);
        }

        delete_the_dead(&mut self.ecs);

        draw_map(&self.ecs, ctx);

        // asks the ECS for read access to where the Position components are stored
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>(); // get map from ECS

        // The join call only returns entities that have both
        for (pos, render) in (&positions, &renderables).join() {
            let i = map.idx(pos.x, pos.y); // get current position
            if map.visible_tiles[i] {
                // render the tile if it's visible
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
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
    let mut gs = State {
        ecs: World::new(),
        run_state: RunState::Running,
    };

    // tell the ECS about the components we made
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<CanMelee>();
    gs.ecs.register::<SuffersDamage>();

    let map: Map = Map::new_map_rooms_and_corridors();

    // Now we can create entities with positions that can also be drawn on the screen
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs
        .create_entity()
        .with(Position::new(player_x, player_y))
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "Hawk Darkstone".to_string(),
        })
        .with(CombatStats::new(30, 30, 2, 5))
        .build();

    // Add a monster to the center of each room
    // skip first room b/c that's where the player spawns
    // add two types of monsters
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let glyph: u8;
        let name: String;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => {
                glyph = rltk::to_cp437('g');
                name = "Goblin".to_string();
            }
            _ => {
                glyph = rltk::to_cp437('o');
                name = "Orc".to_string();
            }
        }

        gs.ecs
            .create_entity()
            .with(Position::new(x, y))
            .with(Renderable {
                glyph: glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{} #{}", &name, i),
            })
            .with(CombatStats::new(16, 16, 1, 4))
            .with(BlocksTile {})
            .build();
    }

    gs.ecs.insert(map);
    // the ECS will be able to track the player's position
    gs.ecs.insert(Point::new(player_x, player_y));

    rltk::main_loop(context, gs);
}
