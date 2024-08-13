/*

*/
use sdl2::sys::SDL_GetTicks64;

mod background;
mod enemy;
mod event;
mod player;

fn main() {
    let sdl_context = sdl2::init().unwrap(); // SDL context

    // subsystems
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();

    let window = video_subsystem
        .window("SDL2 Game Jam - Islands", 1920, 1080)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap(); // canvas wrapper
    let texture_creator = canvas.texture_creator(); // texture creator tied to canvas lifetime
    let mut event_pump = sdl_context.event_pump().unwrap(); // event wrapper

    let mut player = player::Player::new(0, 0, 10, 100);
    let mut background = background::Background::new(&texture_creator);

    let mut enemy_queue = enemy::EnemyQueue::new();
    let mut event_queue = event::EventQueue::new();

    let mut last = timer_subsystem.ticks64();

    'main_loop: loop {
        // update delta time
        let delta = timer_subsystem.ticks64() - last;
        last = timer_subsystem.ticks64();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main_loop,
                _ => {}
            }
        }

        // game logic here
        enemy_queue.move_towards_player(player.rect.x, player.rect.y, delta);
        enemy_queue.check_collisions(player.rect, &mut event_queue);

        // handle events
        event_queue.handle_events(&mut player, &mut enemy_queue);

        canvas.clear();

        // rendering here
        background.render(&mut canvas, 1920, 1080, delta);

        canvas.present();
    }
}
