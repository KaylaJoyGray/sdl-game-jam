/****************************************
 *
 *  Scrolling background
 *
 ****************************************/
use sdl2::rect::Rect;
use sdl2::surface::{Surface, SurfaceRef};
use sdl2::sys::SDL_Surface;
use std::ops::Deref;

pub struct Background<'a> {
    pub speed: f32,
    surface: Surface<'a>,
}

impl<'a> Background<'a> {
    pub fn new(path: &str) -> Self {
        let surface = Surface::load_bmp(path);
        Self {
            speed: 0.1,
            surface: surface.expect("Error: could not load background image"),
        }
    }

    pub fn update() {
        todo!()
    }

    pub fn blit(&self, screen_surface: &mut SurfaceRef) {
        self.surface.blit(
            Rect::new(0, 0, 1920, 1080),
            screen_surface,
            Rect::new(0, 0, 1920, 1080),
        );
    }
}
