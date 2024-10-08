/****************************************
 *
 *  Scrolling background
 *
 ****************************************/
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

pub struct Background<'a> {
    animation_speed: u64,
    delta: u64,
    index: usize,
    textures: Vec<Texture<'a>>,
}

impl<'a> Background<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let mut sfx = 0;
        let mut textures: Vec<Texture<'a>> = Vec::new();

        while sfx < 40 {
            let mut path: String = "assets/background/00".to_string();
            if sfx < 10 {
                path.push_str("0");
            }
            let sfx_str = format!("{}.png", sfx);
            path.push_str(&sfx_str);

            let error = format!("Error: could not load image: {}", path);
            textures.push(texture_creator.load_texture(path).expect(&error));

            sfx += 1;
        }

        Self {
            animation_speed: 100,   // ms
            delta: 0,
            index: 0,
            textures,
        }
    }

    pub fn render(
        &mut self,
        canvas: &mut WindowCanvas,
        screen_width: i32,
        screen_height: i32,
        delta: u64,
    ) {
        // update index
        self.delta += delta;

        if self.delta > self.animation_speed {
            self.index = (self.index + 1) % self.textures.len();
            self.delta = 0;
        }

        let mut y: i32 = 0;
        let mut x: i32 = 0;
        while y < (screen_height) {
            while x < (screen_width) {
                canvas
                    .copy(
                        &self.textures[self.index],
                        None,
                        Rect::new(x, y, 128, 128),
                    )
                    .expect("Error: could not draw background");
                x += 128; // tile size
            }
            y += 128;
            x = 0;
        }
    }
}
