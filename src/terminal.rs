extern crate pancurses;
extern crate time;

use pancurses::*;

// #[derive(Debug)]
// struct Position {
//     x: u32,
//     y: u32
// }
//
// #[derive(Debug)]
// struct Player {
//     position: Position,
//     name: String
// }
//
// #[derive(Debug)]
// struct MainState {
//     players: Vec<Player>
// }
//
fn timestamp() -> f64 {
    let timespec = time::get_time();
    // 1459440009.113178
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    mills
}

fn update() {

}

fn render() {

}

fn main() {
    let window = initscr();

    const FPS: i32 = 10;

    let MS_PER_UPDATE: f64 = 1f64 / FPS as f64;
    let mut previous = timestamp();
    let mut lag = 0f64;

    loop {
        let current = timestamp();
        let elapsed = current - previous;
        previous = current;
        lag += elapsed;

        window.clear();

        window.printw(format!("BEFORE UPDATE"));

        while lag >= MS_PER_UPDATE {
            window.printw(format!("updating {:.4} ... {}", lag, elapsed));
            update();
            lag -= MS_PER_UPDATE;
        }

        window.printw("RENDER");
        window.refresh();
        render();

        // window.printw(format!("Hello Rust {}, el {}", i, dt));
        // window.getch();
    }
    // let pos = Position { x: 3, y: 4 };
    // println!("hehe {:?}", pos);

    endwin();
}

