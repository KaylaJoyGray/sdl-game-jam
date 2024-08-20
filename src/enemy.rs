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
    pub loc: (f32, f32),    // floating point location, used for calculations
    pub rect: Rect,
    pub speed: f32,         // units per ms
    pub sprite_pfx: String, // sprite prefix, i.e. "ghost" for "ghostidle0, ghostdeath0, etc."
    pub state: EnemyState,
    pub index: usize,       // animation index
}

impl Enemy {
    fn new(id: u32, x: i32, y: i32, kind: DamageKind, speed: f32) -> Self {
        Self {
            id,
            kind,
            loc: (x as f32, y as f32),
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

    pub fn add_enemy(&mut self, x: i32, y: i32, kind: DamageKind, speed: f32) {
        self.next_id += 1;
        self.enemies
            .push(Enemy::new(self.next_id, x, y, kind, speed));
    }

    pub fn move_towards_player(&mut self, player_rect: Rect, delta: u64) {
        let delta = delta as f32 / 1000.;

        // find the center of player rect
        let player_x = player_rect.x as u32 + (player_rect.width() / 2);
        let player_y = player_rect.y as u32 + (player_rect.height() / 2);

        for e in &mut self.enemies {
            // calculate movement vector
            let (vx, vy) = (e.loc.0, e.loc.1);
            let (ux, uy) = (player_x as f32, player_y as f32);

            let (mut vecx, mut vecy) = (ux - vx, uy - vy);

            // normalize the vector
            let magnitude = (vecx.powi(2) + vecy.powi(2)).sqrt();
            (vecx, vecy) = (vecx / magnitude, vecy / magnitude);

            if delta != 0. {
                (vecx, vecy) = (vecx * (e.speed / delta), vecy * (e.speed / delta));
            }

            // update location
            e.loc = (e.loc.0 + vecx, e.loc.1 + vecy);

            // update SDL rect
            e.rect.x = e.loc.0 as i32;
            e.rect.y = e.loc.1 as i32;

            //println!("Enemy {} is now at {}, {}", e.id, e.rect.x, e.rect.y);
            println!("Delta: {}", delta);
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
            sprite_str.push_str(&e.index.to_string());
            let texture = self.textures.get(&sprite_str);
            if let Some(texture) = texture {
                canvas
                    .copy(
                        texture,
                        Rect::new(0, 0, 50, 50),
                        Rect::new(e.rect.x, e.rect.y, 10, 10),
                    )
                    .expect("Error: could not draw enemy");
            } else {
                eprintln!("Warning: could not find a texture named {}", sprite_str);
            }
        }
    }
}
