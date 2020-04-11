use rltk::RGB;
use specs::prelude::*;
use specs_derive::*;

/// Entity Position, allows drawing to screen
#[derive(Component)] // makes `Position` a Specs component
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

/// A Component for drawing things to the screen
#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB, // foreground color?
    pub bg: RGB, // background color?
}

// Components with no data are called "tag" components.
#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Monster {}

/*
Adding limited visibility so specific entities can only see the parts of the
 map they've already seen,
 */
#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Debug)]
pub struct BlocksTile {}
