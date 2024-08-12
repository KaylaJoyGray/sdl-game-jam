use crate::enemy::EnemyQueue;
use crate::player::Player;

#[derive(Copy, Clone)]
pub enum DamageKind {}

pub enum Event {
    DamagePlayer(DamageKind),
    KillEnemy(u32), //  enemy ID
}

impl Event {
    fn handle(&self, player: &mut Player, enemy_queue: &mut EnemyQueue) {
        match self {
            Event::DamagePlayer(kind) => player.apply_damage(kind),
            Event::KillEnemy(id) => enemy_queue.kill(id),
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
