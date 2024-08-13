use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::event::{Event, EventQueue};
use crate::event::DamageKind;

struct Enemy {
    id: u32,
    pub alive: bool,
    pub kind: DamageKind,
    pub rect: Rect,
    pub speed: i32, // units per ms
}

impl Enemy {
    fn new(id: u32, x: i32, y: i32, kind: DamageKind, speed: i32) -> Self {
        Self {
            id,
            alive: true,
            kind,
            rect: Rect::new(x, y, 10, 10),
            speed,
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

    pub fn add_enemy(&mut self, x: i32, y: i32, kind: DamageKind, speed: i32) {
        self.next_id += 1;
        self.enemies
            .push(Enemy::new(self.next_id, x, y, kind, speed));
    }

    pub fn move_towards_player(&mut self, player_x: i32, player_y: i32, delta: u64) {
        for e in &mut self.enemies {
            let dist_x = player_x - e.rect.x;
            let dist_y = player_y - e.rect.y;
            let total_dist = ((dist_x.pow(2) + dist_y.pow(2)) as f32).sqrt();

            // Normalize distances and scale by enemy speed and delta time
            let move_x = (dist_x as f32 / total_dist * e.speed as f32 * delta as f32) as i32;
            let move_y = (dist_y as f32 / total_dist * e.speed as f32 * delta as f32) as i32;

            e.rect.x += move_x;
            e.rect.y += move_y;
        }
    }

    pub fn check_collisions(&mut self, player_rect: Rect, event_queue: &mut EventQueue) {
        self.enemies.iter().for_each(|e| {
            if e.rect.has_intersection(player_rect) {
                event_queue.send(Event::DamagePlayer(e.kind)); // send "damage player" event
                event_queue.send(Event::KillEnemy(e.id)); // send "death" event
            }
        });
    }

    pub fn kill(&mut self, id: &u32) {
        self.enemies.retain(|e| e.id != *id);
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas, delta: u64) {
        
    }
}
