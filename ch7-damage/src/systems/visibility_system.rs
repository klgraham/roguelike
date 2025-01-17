// A system to handle map visibility for Entities

use crate::components::{Player, Position, Viewshed};
use crate::map::Map;
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            viewshed.dirty = false; // don't need to update this tile, unless...
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed
                .visible_tiles
                .retain(|p| p.x > 0 && p.x < map.width - 1 && p.y > 0 && p.y < map.height - 1);

            // if this entity is the player, reveal what they can see
            if let Some(_p) = player.get(ent) {
                for t in map.visible_tiles.iter_mut() {
                    *t = false
                }

                for visible in viewshed.visible_tiles.iter() {
                    let i = map.idx(visible.x, visible.y);
                    map.revealed_tiles[i] = true;
                    map.visible_tiles[i] = true;
                }
            }
        }
    }
}
