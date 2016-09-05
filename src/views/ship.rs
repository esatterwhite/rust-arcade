use phi::{Phi,View,ViewAction};
use sdl2::pixels::Color;
use sdl2::rect::Rect as SdlRect;

const PLAYER_SPEED: f64 = 180.0;

struct Ship {
    rect: Rectangle
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    pub fn toSdl( self ) -> Option<SdlRect> {
        assert!(self.w >= 0.0 && self.h >= 0.0 );
        SdlRect::new( self.x as i32, self.y as i32, self.w as u32, self.h as u32 ).unwrap()
    }
}

pub struct ShipView {
    player: Ship
}

impl ShipView {
    pub fn new(phi: &mut Phi) -> Self {
        ShipView {
            player: Ship {
                rect: {
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
    fn render(&mut self, context: &mut Phi, elapsed:f64) -> ViewAction{
        let renderer = &mut context.renderer;
        let events   = &context.events;
        let traveled = PLAYER_SPEED * elapsed

        if events.now.quit || events.now.key_escape == Some(true){
            return ViewAction::Quit
        }
       
        // view logic
        
        renderer.set_draw_color(Color::RGB(0,0,0));
        renderer.clear();

        // rendering logic
        renderer.set_draw_color(Color::RGB(200, 200, 50));
        renderer.fill_rect( self.player.rect.to_sdl().unwrap());

        ViewAction::None
    }
}
