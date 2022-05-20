use macroquad::prelude::*;
use std::{
    thread,
    time::{Duration, Instant},
};
const FPS: f32 = 24.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Survival Pong!".to_owned(),
        window_width: 800,
        window_height: 500,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    main_loop().await;
}

async fn main_loop() {
    let mut x_pos: f32 = 0.0;
    loop {
        let now = Instant::now();
        clear_background(BLACK);
        let margin = 3.0;
        draw_rectangle_lines(
            margin,
            margin,
            screen_width() - margin,
            screen_height() - margin,
            2.0,
            GREEN,
        );
        let fps = format!("{:.2}fps", 1.0 / get_frame_time());
        draw_text(&fps, 30.0, 30.0, 30.0, GREEN);
        x_pos += 2.0;
        if x_pos > screen_width() + 10.0 {
            x_pos = -10.0;
        }
        let y_pos = screen_height() / 2.0;
        let size = 10.0;
        let size_2 = size / 2.0;

        draw_rectangle(x_pos - size_2, y_pos - size_2, size, size, WHITE);

        // --- WAIT LOGIC FOR SMOTHNESS ----
        next_frame().await;
        let elapsed = now.elapsed().as_secs_f32();
        let want_frame_time = 1.0 / FPS;
        let sleep_time: f32 = want_frame_time - elapsed;
        if sleep_time > 0.0 {
            thread::sleep(Duration::from_secs_f32(sleep_time));
        }
    }
}
