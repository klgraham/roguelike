use super::{Map, Player, Position, State, TileType, Viewshed};
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        // Can't walk through walls
        if map[(pos.x + dx, pos.y + dy)] != TileType::Wall {
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
