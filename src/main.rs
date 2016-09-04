extern crate sdl2;

mod phi;
mod views;

use phi::{Events, Phi, View, ViewAction};


fn main() {

    // initialize SDL2
    let sdl_context =  sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();

    // create a new window
    let window = video.window("Arcade Shooter", 800, 600)
            .position_centered().opengl()
            .build().unwrap();

    let mut context = Phi{
        events: Events::new( sdl_context.event_pump().unwrap() ),
        renderer: window.renderer().accelerated().build().unwrap(),
    };

    let mut current_view: Box<View> = Box::new(self::views::DefaultView);

    // Frame timing

    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;


    loop {
        let now = timer.ticks();
        let delta = now - before;
        let elapsed = delta as f64 / 1_000.0;


        if delta < interval {
            timer.delay( interval - delta );
                continue;
        }

        before = now;
        fps += 1;

       if now - last_second > 1_000 {
           println!("FPS: {}", fps);
           last_second = now;
           fps = 0;
       }

        context.events.pump();
        match current_view.render( &mut context, elapsed ){
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => break
        }
    }
}
