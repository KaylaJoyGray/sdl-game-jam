use crate::event::DamageKind;
use sdl2::rect::Rect;

pub struct Player {
    pub rect: Rect,
}

impl Player {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height)
        }
    }

    pub fn apply_damage(&mut self, kind: &DamageKind) {
        /*todo!()*/
    }
}
