use core::time;
use macroquad::prelude::*;
use miniquad::conf::Platform;
use std::env;

const DEFAULT_SIZE: usize = 100;
const DISPLAY_FACTOR: f32 = 1.0;
const MAX_STEPS: usize = 1000;

fn window_conf() -> Conf {
    Conf {
        window_resizable: false,
        platform: Platform {
            swap_interval: Some(0), // disable vsync
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // TODO: state initialization is considered in timing
    let args: Vec<String> = env::args().collect();
    let n: usize = match args.get(1) {
        Some(value) => value.parse::<usize>().expect("expect an integer value"),
        None => DEFAULT_SIZE,
    };
    let mut steps: usize = 0;

    loop {
        clear_background(WHITE);

        steps += 1;
        if steps > MAX_STEPS {
            let time = macroquad::time::get_time();
            println!("{:}", time);
            return; // exit
        }

        for x in 0..n {
            for y in 0..n {
                draw_rectangle(
                    (x as f32) * DISPLAY_FACTOR,
                    (y as f32) * DISPLAY_FACTOR,
                    DISPLAY_FACTOR,
                    DISPLAY_FACTOR,
                    match (x + y) % 2 == 0 {
                        true => WHITE,
                        false => GRAY,
                    },
                )
            }
        }

        next_frame().await
    }
}
