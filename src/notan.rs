use std::env;

use notan::draw::*;
use notan::prelude::*;

const DEFAULT_SIZE: usize = 100;
const DISPLAY_FACTOR: f32 = 1.0;
const MAX_STEPS: usize = 1000;

#[derive(AppState)]
struct State {
    n: usize,
    steps: usize,
}

impl Default for State {
    fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let n: usize = match args.get(1) {
            Some(value) => value.parse::<usize>().expect("expect an integer value"),
            None => DEFAULT_SIZE,
        };

        State { n, steps: 0 }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::default)
        .draw(draw)
        .add_config(DrawConfig)
        .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    state.steps += 1;
    if state.steps > MAX_STEPS {
        let time = app.timer.elapsed_f32();
        let time2 = app.system_timer.elapsed_f32();
        println!("{:} {:}", time, time2);
        app.backend.exit();
    }

    let mut draw = gfx.create_draw();
    draw.clear(Color::WHITE);
    for x in 0..state.n {
        for y in 0..state.n {
            draw.rect(
                ((x as f32) * DISPLAY_FACTOR, (y as f32) * DISPLAY_FACTOR),
                (DISPLAY_FACTOR, DISPLAY_FACTOR),
            )
            .fill_color(match (x + y) % 2 == 0 {
                true => Color::WHITE,
                false => Color::GRAY,
            });
        }
    }
    gfx.render(&draw);
}
