// Behavior for monsters

use crate::components::{Monster, Name, Position, Viewshed};
use crate::map::Map;
use rltk::{console, DistanceAlg, Point};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    // tell the linter to ignore type complexity of SystemData
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_position, mut viewshed, monster, name, mut position) = data;

        for (mut viewshed, _monster, name, mut pos) in
            (&mut viewshed, &monster, &name, &mut position).join()
        {
            let p = Point::new(pos.x, pos.y);
            let distance = DistanceAlg::Pythagoras.distance2d(p, *player_position);

            if distance < 1.5 {
                // attack happens here
                let message = format!("{} prepares for battle.", name.name);
                console::log(&message);
                return;
            }

            // if true, the player position is visible to the monster
            if viewshed.visible_tiles.contains(&*player_position) {
                // A* search for path from monster to player
                let path = rltk::a_star_search(
                    map.idx(pos.x, pos.y) as i32,
                    map.idx(player_position.x, player_position.y) as i32,
                    &mut *map,
                );

                if path.success && path.steps.len() > 1 {
                    pos.x = path.steps[1] as i32 % map.width;
                    pos.y = path.steps[1] as i32 / map.width;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
