/****************************************
 *
 *  Scrolling background
 *
 ****************************************/
use sdl2::surface::Surface;

struct Background<'a> {
    speed: f32,
    surface: &'a Surface<'a>,
}

impl<'a> Background<'a>  {

    fn present(&self) {
        todo!()
    }
}