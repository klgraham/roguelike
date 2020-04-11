// Behavior for monsters

// extern crate specs;
use specs::prelude::*;
use super::{Monster, Viewshed, Name};
// extern crate rltk;
use rltk::{Point, console};


pub struct MonsterAI {}

impl <'a> System<'a> for MonsterAI {
    type SystemData = ( ReadExpect<'a, Point>,
                        ReadStorage<'a, Viewshed>,
                        ReadStorage<'a, Monster>,                        
                        ReadStorage<'a, Name>);

    fn run(&mut self, data: Self::SystemData) {
        let (player_position, viewshed, monster, name) = data;

        for (view, _monster, name) in (&viewshed, &monster, &name).join() {
            // if true, the player position is visible to the monster
            if view.visible_tiles.contains(&*player_position) {
                let message = format!("{} prepares for battle.", name.name);
                console::log(&message);
            }
        }
    }
}