use crate::event::DamageKind;

pub struct Player {}

impl Player {
    pub fn new() -> Self {
        Self {}
    }

    pub fn apply_damage(&mut self, kind: DamageKind) {
        todo!()
    }
}
