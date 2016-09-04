extern crate sdl2;

mod phi;
mod views;

fn main() {
    self::phi::spawn("Arcade Shooter", |_|{
        Box::new(self::views::DefaultView)
    });
}
