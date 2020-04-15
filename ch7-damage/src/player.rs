use crate::components::{CanMelee, CombatStats, Player, Position, Viewshed};
use crate::map::Map;
use super::{RunState, State};
use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    /*
    Since we want to update the player's position and the tiles they have
    visited we need write access from the ECS.
    */

    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let mut wants_to_melee = ecs.write_storage::<CanMelee>();
    let entities = ecs.entities();

    let map = ecs.fetch::<Map>(); // fetch the Map so we can update it

    /*  */
    for (entity, _player, pos, viewshed) in
        (&entities, &players, &mut positions, &mut viewsheds).join()
    {
        if !map.contains(pos.x + dx, pos.y + dy) {
            return;
        }
        let destination = map.idx(pos.x + dx, pos.y + dy);

        for potential_target in map.tile_content[destination].iter() {
            if let Some(target) = combat_stats.get(*potential_target) {
                wants_to_melee
                    .insert(
                        entity,
                        CanMelee {
                            target: *potential_target,
                        },
                    )
                    .expect("Add target failed.");
            }
        }

        if !map.blocked[destination] {
            // Can't walk through walls
            pos.x = min(79, max(0, pos.x + dx));
            pos.y = min(49, max(0, pos.y + dy));
            viewshed.dirty = true;

            // When the player moves, update the position in the ECS
            let mut player_position = ecs.write_resource::<Point>();
            player_position.x = pos.x;
            player_position.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => return RunState::Paused, // nothing happened
        Some(key) => match key {
            // Using standard PC gaming movement controls
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs), // up
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs), // left
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),  // down
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),  // right

            // diagonals
            VirtualKeyCode::Q => try_move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::Z => try_move_player(-1, 1, &mut gs.ecs),
            VirtualKeyCode::E => try_move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::X => try_move_player(1, 1, &mut gs.ecs),
            // VirtualKeyCode::Numpad4 => try_move_player(-1, 0, &mut gs.ecs),
            // VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),
            // VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            // VirtualKeyCode::Numpad6 => try_move_player(1, 0, &mut gs.ecs),
            // VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),
            // VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            // VirtualKeyCode::Numpad8 => try_move_player(0, -1, &mut gs.ecs),
            // VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),
            // VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            // VirtualKeyCode::Numpad2 => try_move_player(0, 1, &mut gs.ecs),
            // VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),
            _ => return RunState::Paused, // do nothing if any other key is hit
        },
    }

    RunState::Running
}
