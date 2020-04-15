use crate::components::{BlocksTile, Position};
use crate::map::Map;
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers, entities) = data;

        map.populate_blocked();
        map.clear_content_index();
        for (entity, position) in (&entities, &position).join() {
            let i = map.idx(position.x, position.y);

            // if this entity has a blocks tile component,
            // update the map's blocking list
            let _p: Option<&BlocksTile> = blockers.get(entity);
            if let Some(_p) = _p {
                map.blocked[i] = true;
            }

            // add this entity to the tile content index for ith tile
            map.tile_content[i].push(entity);
        }
    }
}
