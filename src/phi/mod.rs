extern crate sdl2;

#[macro_use]
mod events;

use sdl2::render::Renderer;

pub fn spawn<F>(title: &str, init:F) where F: Fn( &mut Phi ) -> Box<View> {


    // initialize SDL2
    let sdl_context =  sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();

    // create a new window
    let window = video.window(&title, 800, 600)
            .position_centered().opengl().resizable()
            .build().unwrap();

    let mut context = Phi{
        events: Events::new( sdl_context.event_pump().unwrap() ),
        renderer: window.renderer().accelerated().build().unwrap(),
    };

    let mut current_view = init(&mut context);


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

        // pump spins the game loop 1 tick
        context.events.pump( &mut context.renderer );

        // render returns a view action, which we match on
        match current_view.render( &mut context, elapsed ){
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => break,
            ViewAction::ChangeView(new_view) => current_view = new_view,
        }
    }

}

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_space: Space,
        key_left: Left,
        key_right: Right
    },
    else: {
        quit: Quit { .. }
    }
}

/// Bundles the Phi abstractions in a single structur
/// to be passed easily
pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>
}

impl<`window> Phi<`window> {
    pub fn output_size(&self) -> ( f64, f64 ){
        let ( h, w ) = self.renderer.output_size().unwrap();
        ( w as f64, h as f64 )
    }
}

/// A Way for the currently executed View
/// to communicate with the game loop. It specifies
/// which action should be executed before the next rendering
pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>)
}

pub trait View {
    /// Called every fram to take care of both the logic
    /// and the rendering of the current view
    ///
    /// `elapsed` is expressed in seconds
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

