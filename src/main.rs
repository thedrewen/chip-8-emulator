use std::env::args;

use crate::screen::Screen;

mod computer;
mod keyboard;
mod screen;
mod sound;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        println!("Please specify the ROM.");
        return;
    }

    let mut screen = Screen::new(args[1].clone());
    screen.play();
}
