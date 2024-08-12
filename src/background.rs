/****************************************
 *
 *  Scrolling background
 *
 ****************************************/
use sdl2::rect::Rect;
use sdl2::surface::{Surface, SurfaceRef};

pub struct Background<'a> {
    pub speed: f32,
    index: usize,
    surfaces: Vec<Surface<'a>>,
}

impl<'a> Background<'a> {
    pub fn new() -> Self {
        let mut sfx = 0;
        let mut surfaces: Vec<Surface<'a>> = Vec::new();

        while sfx < 40 {
            let mut path: String = "assets/background/00".to_string();
            if sfx < 10 {
                path.push_str("0");
            }
            let sfx_str = format!("{}.png", sfx);
            path.push_str(&sfx_str);

            let error = format!("Error: could not load image: {}", path);
            surfaces.push(Surface::load_bmp(path).expect(&error));

            sfx += 1;
        }

        Self {
            speed: 0.1,
            index: 0,
            surfaces,
        }
    }

    pub fn update() {
        todo!()
    }

    pub fn blit(&mut self, screen_surface: &mut SurfaceRef, screen_width: u32, screen_height: u32, delta: f32) {
        // update index
        let add = (delta % self.speed) as usize;
        self.index = (self.index + add) % self.surfaces.len();

        let mut y: i32 = (0 - (screen_height / 2)) as i32;
        let mut x: i32 = (0 - (screen_width / 2)) as i32;
        while y < (screen_height / 2) as i32 {
            while x < (screen_width / 2) as i32 {
                self.surfaces[self.index]
                    .blit(
                        Rect::new(0, 0, 128, 128),
                        screen_surface,
                        Rect::new(x, y, 1920, 1080),
                    )
                    .expect("Error: could not draw background");
                x += 128;   // tile size
            }
            y += 128;
            x = (0 - (screen_width / 2)) as i32;
        }
    }
}
