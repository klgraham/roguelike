use super::{xy_idx, Player, Position, State, TileType};
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination = xy_idx(pos.x + dx, pos.y + dy);

        // Can't walk through walls
        if map[destination] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + dx));
            pos.y = min(49, max(0, pos.y + dy));
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
