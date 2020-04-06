use super::{Map, Player, Position, State, TileType, Viewshed};
use rltk::{Rltk, VirtualKeyCode};
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
    let map = ecs.fetch::<Map>(); // fetch the Map so we can update it

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        if map[(pos.x + dx, pos.y + dy)] != TileType::Wall {
            // Can't walk through walls
            pos.x = min(79, max(0, pos.x + dx));
            pos.y = min(49, max(0, pos.y + dy));
            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {} // nothing happens
        Some(key) => match key {
            // Using standard PC gaming movement controls
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            // VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
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
            _ => {} // do nothing if any other key is hit
        },
    }
}
