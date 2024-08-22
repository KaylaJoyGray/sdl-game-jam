use crate::enemy::EnemyQueue;
use crate::player::Player;

#[derive(Copy, Clone)]
pub enum DamageKind {
    Normal(u32),    // health to take
}

pub fn damage_health(health: &mut i32, kind: DamageKind) {
    match kind {
        DamageKind::Normal(amount) => {
            *health -= amount as i32;
        }
    }
}

pub enum Event {
    DamagePlayer(DamageKind),
    DamageEnemy(u32, DamageKind), // enemy ID, DamageKind
    KillEnemy(u32), //  enemy ID
}

impl Event {
    fn handle(&self, player: &mut Player, enemy_queue: &mut EnemyQueue) {
        match self {
            Event::DamagePlayer(kind) => player.apply_damage(*kind),
            Event::KillEnemy(id) => enemy_queue.kill(id),
            Event::DamageEnemy(id, kind) => enemy_queue.damage_enemy(*id, *kind),
        }
    }
}

pub struct EventQueue {
    events: Vec<Event>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn send(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn handle_events(&mut self, player: &mut Player, enemy_queue: &mut EnemyQueue) {
        for e in &mut self.events {
            e.handle(player, enemy_queue);
        }
    }
}
