extern crate sdl2;

mod phi;
mod views;

fn main() {
    self::phi::spawn("Arcade Shooter", |ctx|{
        Box::new(self::views::ship::ShipView::new(ctx))
    });
}
