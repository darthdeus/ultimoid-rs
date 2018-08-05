// #[macro_use]
extern crate itertools;
extern crate piston_window;
extern crate graphics;
extern crate cgmath;

mod map;

use map::*;
use piston_window::*;
use graphics::math::Matrix2d;
use piston_window::Button::Keyboard;
use piston_window::MouseButton;
use piston_window::Key;
use cgmath::*;

fn main() {
    let screen_size = vec2(640f64, 480f64);

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [screen_size.x as u32, screen_size.y as u32])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut player_coord = Coord::new(0, 0);

    let mut rmb_pressed = false;
    let mut move_acc = 0;
    let mut cursor = ScreenPoint::zero();

    let iso_rot = graphics::math::rotate_radians(3.14 / 4.0);

    while let Some(event) = window.next() {
//        if let Some(ref args) = e.render_args() {
//        }

        event.mouse_cursor(|x, y| {
//            let [x, y] = graphics::math::transform_vec(graphics::math::rotate_radians(3.14 / 4.0)
//                                                       , [x, -y]);
            cursor = ScreenPoint::new(x, y);
        });

        if let Some(Button::Mouse(MouseButton::Right)) = event.press_args() {
            rmb_pressed = true;
            println!("PRESSED");
        };

        if let Some(Button::Mouse(MouseButton::Right)) = event.release_args() {
            println!("released");
            rmb_pressed = false;
            move_acc = 0;
        };

        if rmb_pressed { move_acc += 1; }

//        println!("rmb {}, move acc {}", rmb_pressed, move_acc);

        match event.press_args() {
            Some(Keyboard(Key::Left)) => { player_coord.x -= 1 }
            Some(Keyboard(Key::Right)) => { player_coord.x += 1 }
            Some(Keyboard(Key::Up)) => { player_coord.y += 1 }
            Some(Keyboard(Key::Down)) => { player_coord.y -= 1 }
            _ => {}
        };

        let mobs = vec![Coord::new(0, 1), Coord::new(-1, 3)];

        let grid_size = 20.0f64;
        let grid_spacing = 2.0f64;

        let coord_to_screen_point = |coord: Coord| {
            let rect_x: f64 = (coord.x as f64) * (grid_size + grid_spacing);
            let rect_y: f64 = (coord.y as f64) * (grid_size + grid_spacing);

            vec2(rect_x, rect_y)
        };

        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            let player_point = coord_to_screen_point(player_coord);

            let offset = -player_point + screen_size / 2;

            let trans = graphics::math::multiply(iso_rot,
                                                 context.transform);

            let trans = context.transform;

            for x in -10..10 {
                for y in -10..10 {
                    let draw_point = coord_to_screen_point(Coord::new(x, y)) + offset;

                    rectangle([0.5, 0.5, 0.5, 1.0],
                              [draw_point.x, draw_point.y, grid_size, grid_size],
                              trans,
                              graphics);
                }
            }

            let player_draw_at = player_point + offset;

            let direction = (cursor - player_draw_at).flip_y();

            if move_acc >= 110 {
                if direction.x > 0f64 {
                    if direction.x > direction.y.abs() {
                        player_coord.x += 1;
                    } else if direction.y > 0f64 {
                        player_coord.y -= 1;
                    } else {
                        player_coord.y += 1;
                    }
                } else {
                    if direction.x.abs() > direction.y.abs() {
                        player_coord.x -= 1;
                    } else if direction.y > 0f64 {
                        player_coord.y -= 1;
                    } else {
                        player_coord.y += 1;
                    }
                }
//                if direction.x < 0f64 {
//                    player_coord.x -= 1;
//                } else {
//                    player_coord.x += 1;
//                }

                move_acc = 0;
            }

            rectangle([1.0, 0.5, 0.5, 1.0],
                      [player_draw_at.x, player_draw_at.y, grid_size, grid_size],
                      trans,
                      graphics);

            // rectangle([1.0, 0.0, 0.0, 1.0], // red
            //           [0.0, 0.0, 100.0, 100.0],
            //           context.transform,
            //           graphics);
        });
    }
}
