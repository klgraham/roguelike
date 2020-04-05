use specs::prelude::*;
use specs_derive::*;
use rltk::RGB;


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