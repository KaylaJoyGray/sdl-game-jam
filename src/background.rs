/****************************************
 *
 *  Scrolling background
 *
 ****************************************/
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::SurfaceRef;
use sdl2::video::WindowContext;

pub struct Background<'a> {
    pub speed: f32,
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
            speed: 0.1,
            index: 0,
            textures,
        }
    }

    pub fn update() {
        todo!()
    }

    pub fn blit(
        &mut self,
        canvas: &mut WindowCanvas,
        screen_width: u32,
        screen_height: u32,
        delta: f32,
    ) {
        // update index
        let add = (delta % self.speed) as usize;
        self.index = (self.index + add) % self.textures.len();

        let mut y: i32 = (0 - (screen_height / 2)) as i32;
        let mut x: i32 = (0 - (screen_width / 2)) as i32;
        while y < (screen_height / 2) as i32 {
            while x < (screen_width / 2) as i32 {
                canvas
                    .copy(
                        &self.textures[self.index],
                        None,
                        Rect::new(x, y, 1920, 1080),
                    )
                    .expect("Error: could not draw background");
                x += 128; // tile size
            }
            y += 128;
            x = (0 - (screen_width / 2)) as i32;
        }
    }
}
