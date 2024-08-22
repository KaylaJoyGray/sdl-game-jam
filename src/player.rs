use sdl2::rect::Rect;

use crate::event::{damage_health, DamageKind};

pub struct Player {
    health: i32,
    pub rect: Rect,
}

impl Player {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            health: 100,
            rect: Rect::new(x, y, width, height),
        }
    }

    pub fn apply_damage(&mut self, kind: DamageKind) {
        damage_health(&mut self.health, kind);
    }
}
