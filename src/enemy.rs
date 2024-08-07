use crate::event::{Event, EventQueue};
use sdl2::rect::Rect;
use crate::damage::DamageKind;

struct Enemy {
    id: u32,
    pub alive: bool,
    pub kind: DamageKind,
    pub rect: Rect,
}

impl Enemy {
    fn new(id: u32, x: i32, y: i32, kind: DamageKind) -> Self {
        Self {
            id,
            alive: true,
            kind,
            rect: Rect::new(x, y, 10, 10),
        }
    }
}

pub struct EnemyQueue {
    next_id: u32,
    enemies: Vec<Enemy>,
}

impl EnemyQueue {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            enemies: Vec::new(),
        }
    }

    pub fn add_enemy(&mut self, x: i32, y: i32, kind: DamageKind) {
        self.next_id += 1;
        self.enemies.push(Enemy::new(self.next_id, x, y, kind));
    }

    pub fn move_towards_player(&self, player_x: i32, player_y: i32, delta: f32) {
        todo!()
    }

    pub fn check_collisions(&mut self, player_rect: Rect, event_queue: &mut EventQueue) {
        self.enemies.iter().for_each(|e| {
            if e.rect.has_intersection(player_rect) {
                event_queue.send(Event::DamagePlayer(e.kind)); // send "damage player" event
                event_queue.send(Event::KillEnemy(e.id)); // send "death" event
            }
        });
    }

    pub fn kill(&mut self, id: u32) {
        self.enemies.retain(|e| e.id != id);
    }
}
