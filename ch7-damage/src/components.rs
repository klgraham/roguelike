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

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

impl CombatStats {
    pub fn new(max_hp: i32, hp: i32, defense: i32, power: i32) -> CombatStats {
        CombatStats {
            max_hp,
            hp,
            defense,
            power,
        }
    }
}

/// Component that gives an entity the ability to do mêlée damage
#[derive(Component, Debug, Clone)]
pub struct CanMelee {
    pub target: Entity,
}

/// Component that tracks the incoming damage suffered by an entity
#[derive(Component, Debug)]
pub struct SuffersDamage {
    pub amount: Vec<i32>,
}

impl SuffersDamage {
    pub fn new_damage(store: &mut WriteStorage<SuffersDamage>, victim: Entity, amount: i32) {
        if let Some(incoming_damage) = store.get_mut(victim) {
            incoming_damage.amount.push(amount);
        } else {
            let damage = SuffersDamage {
                amount: vec![amount],
            };
            store
                .insert(victim, damage)
                .expect("Unable to insert damage.");
        }
    }
}
