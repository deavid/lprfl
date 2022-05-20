use macroquad::{
    audio::{load_sound, play_sound_once, set_sound_volume},
    prelude::*,
};
use std::{
    thread,
    time::{Duration, Instant},
};
const FPS: f32 = 30.0;

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
    // TODO: Cut sounds
    let click = load_sound("sfx/click.wav").await.unwrap();
    let snare = load_sound("sfx/snare.wav").await.unwrap();
    let over = load_sound("sfx/over.wav").await.unwrap();
    set_sound_volume(click, 2.0);
    set_sound_volume(snare, 2.0);
    set_sound_volume(over, 1.0);
    let mut score = 0;
    // BALL
    let mut x_pos: f32 = screen_width() - 50.0;
    let mut y_pos = screen_height() / 2.0;
    let mut dx = -5.0;
    let mut dy = 4.0;
    let speed_mult = 1.1;
    // PADDLE
    let paddle_x: f32 = 30.0;
    let mut paddle_y: f32 = screen_height() / 2.0;
    // BALL SIZES
    let size = 10.0;
    let size_2 = size / 2.0;
    // PADDLE SIZES
    let sizex = 10.0;
    let mut sizey = 150.0; // shortens over time.
    let sizex_2 = sizex / 2.0;
    let mut sizey_2 = sizey / 2.0;

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
        let scr_margin = 20.0;

        // BALL MOVEMENT

        let prec_steps = 20;
        for _ in 0..prec_steps {
            x_pos += dx / prec_steps as f32;
            y_pos += dy / prec_steps as f32;

            // DOES BALL HIT WALL?
            if y_pos - size_2 < scr_margin {
                y_pos = scr_margin + size_2;
                dy = dy.abs();
                score += 1;
                play_sound_once(click);
            }
            if y_pos + size_2 > screen_height() - scr_margin {
                y_pos = screen_height() - scr_margin - size_2;
                dy = -dy.abs();
                score += 1;
                play_sound_once(click);
            }
            if x_pos + size_2 < 0.0 {
                // FOUL! Init the velocities and position
                dx = -5.0;
                dy = 4.0;
                x_pos = screen_width() - 50.0;
                y_pos = screen_height() / 2.0;
                score /= 2;
                sizey = 150.0;
                sizey_2 = sizey / 2.0;
                play_sound_once(over);
            }
            if x_pos + size_2 > screen_width() - scr_margin {
                x_pos = screen_width() - scr_margin - size_2;
                dx = -dx.abs();
                score += 1;
                play_sound_once(click);
            }

            // DOES BALL HIT PADDLE?
            if x_pos - size_2 < paddle_x + sizex_2 && x_pos + size_2 > paddle_x - sizex_2 {
                // In the same X.
                if y_pos - size_2 < paddle_y + sizey_2 && y_pos + size_2 > paddle_y - sizey_2 {
                    // In the same Y
                    if dx < 0.0 {
                        dx = dx.abs();
                        let speed = f32::sqrt(dx * dx + dy * dy) * speed_mult;
                        let diff_y = (y_pos - paddle_y) / sizey_2;
                        dy += diff_y * 5.0;
                        let speed2 = f32::sqrt(dx * dx + dy * dy);
                        let f = speed / speed2;
                        dx *= f;
                        dy *= f;
                        score += 1;
                        sizey /= 1.05;
                        sizey_2 = sizey / 2.0;
                        play_sound_once(snare);
                    }
                }
            }
        }
        // BALL RENDERING
        draw_rectangle(x_pos - size_2, y_pos - size_2, size, size, WHITE);

        // KEYBOARD INPUT - TO PADDLE
        let up = is_key_down(KeyCode::W);
        let down = is_key_down(KeyCode::S);
        if up {
            paddle_y -= 6.0;
        }
        if down {
            paddle_y += 6.0;
        }
        if paddle_y - sizey_2 < scr_margin {
            paddle_y = scr_margin + sizey_2;
        }
        if paddle_y + sizey_2 > screen_height() - scr_margin {
            paddle_y = screen_height() - scr_margin - sizey_2;
        }
        // PADDLE RENDERING

        draw_rectangle(paddle_x - sizex_2, paddle_y - sizey_2, sizex, sizey, YELLOW);

        // UI - FPS METERS
        let fps = format!("{:.2}fps", 1.0 / get_frame_time());
        draw_text(&fps, 600.0, 30.0, 30.0, GREEN);

        let score = format!("SCORE: {:05} POINTS", score);
        draw_text(&score, 20.0, 30.0, 30.0, SKYBLUE);

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
