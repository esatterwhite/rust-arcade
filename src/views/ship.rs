use phi::{Phi,View,ViewAction};
use phi::data::Rectangle;
use sdl2::pixels::Color;

const PLAYER_SPEED: f64 = 180.0;

struct Ship {
    rect: Rectangle
}
pub struct ShipView {
    player: Ship
}

/// Ship view
impl ShipView {
    /// Function new
    pub fn new(phi: &mut Phi) -> Self {
        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: 32.0,
                    h: 32.0
                }
            }
        }
    }
}


impl View for ShipView {
    /// arcade method
    fn render(&mut self, context: &mut Phi, elapsed:f64) -> ViewAction{
        let events   = &context.events;
        let traveled = PLAYER_SPEED * elapsed;
        if events.now.quit || events.now.key_escape == Some(true){
            return ViewAction::Quit
        }
       
        let diagonal = 
            ( context.events.key_up ^ context.events.key_down) &&
            ( context.events.key_left ^ context.events.key_right);

        let moved =
            if diagonal { 1.0 / 2.0f64.sqrt() }
            else { 1.0 } * PLAYER_SPEED * elapsed;

        let dy = match ( context.events.key_up, context.events.key_down ) {
            ( true, true ) | ( false, false ) => 0.0,
            ( true, false ) => -moved,
            ( false, true ) => moved,
        };

        let dx = match ( context.events.key_left, context.events.key_right ) {
            ( true, true ) | ( false, false ) => 0.0,
            ( true, false ) => -moved,
            ( false, true ) => moved,
        };

        let bounding_box = Rectangle {
            x: 0.0,
            y: 0.0,
            w: context.output_size().0 * 0.50,
            h: context.output_size().1,
        };
        
        self.player.rect.x += dx;
        self.player.rect.y += dy;

        println!("y: {}, dy:{}", self.player.rect.y, dy);         
        // view logic
        
        context.renderer.set_draw_color(Color::RGB(0,0,0));
        context.renderer.clear();

        // rendering logic
        context.renderer.set_draw_color(Color::RGB(200, 200, 50));
        context.renderer.fill_rect( self.player.rect.to_sdl().unwrap());

        self.player.rect = self.player.rect.move_inside( bounding_box ).unwrap();
        ViewAction::None
    }
}
