extern crate sdl2;

#[macro_use]
mod events;

use sdl2::render::Renderer;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down
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

/// A Way for the currently executed View
/// to communicate with the game loop. It specifies
/// which action should be executed before the next rendering
pub enum ViewAction {
    None,
    Quit,
}

pub trait View {
    /// Called every fram to take care of both the logic
    /// and the rendering of the current view
    ///
    /// `elapsed` is expressed in seconds
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

