extern crate ggez;

use ggez::*;
use ggez::graphics::{DrawMode, Point2, Rect};

struct Position {
    x: u32,
    y: u32
}

struct Player {
    position: Position,
    name: String
}

struct MainState {
    pos_x: f32,
    players: Vec<Player>
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let p1 = Player {
            name: "john".to_string(),
            position: Position { x: 3, y: 4 }
        };
        let s = MainState { pos_x: 0.0, players: vec![p1] };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 2.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        for x in 0..10 {
            for y in 0..10 {
                const grid_size: f32 = 20.0;

                let rect = Rect::new((grid_size + 1f32) * x as f32, (grid_size + 1f32) * y as f32, grid_size, grid_size);
                graphics::rectangle(ctx, DrawMode::Fill, rect);
            }
        }
        graphics::circle(ctx,
                         DrawMode::Fill,
                         Point2::new(self.pos_x, 380.0),
                         100.0,
                         2.0)?;
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}

