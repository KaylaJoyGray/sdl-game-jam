use std::collections::HashMap;

use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use crate::event::{Event, EventQueue};
use crate::event::DamageKind;

enum EnemyState {
    Idle,
    Attack,
    Dying,
}

struct Enemy {
    id: u32,
    pub kind: DamageKind,
    pub rect: Rect,
    pub speed: i32,         // units per ms
    pub sprite_pfx: String, // sprite prefix, i.e. "ghost" for "ghostidle0, ghostdeath0, etc."
    pub state: EnemyState,
    pub index: usize,       // animation index
}

impl Enemy {
    fn new(id: u32, x: i32, y: i32, kind: DamageKind, speed: i32) -> Self {
        Self {
            id,
            kind,
            rect: Rect::new(x, y, 10, 10),
            speed,
            sprite_pfx: "ghost".to_string(),
            state: EnemyState::Idle,
            index: 0,
        }
    }
}

pub struct EnemyQueue<'a> {
    next_id: u32,
    enemies: Vec<Enemy>,
    textures: HashMap<String, Texture<'a>>,
}

impl<'a> EnemyQueue<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let mut textures = HashMap::<String, Texture<'a>>::new();
        textures.insert(
            "ghostidle0".to_string(),
            texture_creator
                .load_texture("assets/enemies/ghostidle/sprite_0.png")
                .expect("Could not load ghostidle/sprite_0.png"),
        );

        Self {
            next_id: 0,
            enemies: Vec::new(),
            textures,
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

    pub fn render(&self, canvas: &mut WindowCanvas) {
        for e in &self.enemies {
            let mut sprite_str = e.sprite_pfx.clone();
            match e.state {
                EnemyState::Idle => {
                    sprite_str.push_str("idle");
                }
                EnemyState::Attack => {
                    sprite_str.push_str("attack");
                }
                EnemyState::Dying => {
                    sprite_str.push_str("death");
                }
            }
            let texture = self.textures.get(&sprite_str);
            if let Some(texture) = texture {
                canvas
                    .copy(
                        texture,
                        Rect::new(0, 0, 50, 50),
                        Rect::new(e.rect.x, e.rect.y, 50, 50),
                    )
                    .expect("Error: could not draw enemy");
            } else {
                eprintln!("Warning: could not find a texture named {}", e.sprite_pfx);
            }
        }
    }
}
