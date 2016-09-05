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
    pub fn to_sdl( self ) -> Option<SdlRect> {
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
    fn render(&mut self, context: &mut Phi, elapsed:f64) -> ViewAction{
        let renderer = &mut context.renderer;
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

        self.player.rect.x += dx;
        self.player.rect.y += dy;
       
        // view logic
        
        renderer.set_draw_color(Color::RGB(0,0,0));
        renderer.clear();

        // rendering logic
        renderer.set_draw_color(Color::RGB(200, 200, 50));
        renderer.fill_rect( self.player.rect.to_sdl().unwrap());

        ViewAction::None
    }
}
