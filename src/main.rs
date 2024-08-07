/*

*/

mod enemy;
mod event;
mod player;

fn main() {
    let sdl_context = sdl2::init().unwrap(); // SDL context
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("SDL2 Game Jam - Islands", 1920, 1080)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap(); // canvas wrapper
    let mut event_pump = sdl_context.event_pump().unwrap(); // event wrapper
    
    let enemy_queue = enemy::EnemyQueue::new();

    canvas.clear();
    canvas.present();
}
