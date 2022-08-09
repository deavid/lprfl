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
    // Sometimes sound does not work...
    let click = load_sound("sfx/click.wav").await.unwrap();
    let snare = load_sound("sfx/snare.wav").await.unwrap();
    let over = load_sound("sfx/over.wav").await.unwrap();
    // SOUND VOLUME
    set_sound_volume(click, 2.0);
    set_sound_volume(snare, 2.0);
    set_sound_volume(over, 2.0);

    let mut score = 0;
    // BALL POSITION
    let mut ball_pos_x: f32 = screen_width() - 50.0;
    let mut ball_pos_y = screen_height() / 2.0;
    let mut ball_direction_x = -5.0;
    let mut ball_direction_y = 4.0;
    let speed_mult = 1.1;
    // PADDLE POSITION
    let paddle_pos_x: f32 = 30.0;
    let mut paddle_pos_y: f32 = screen_height() / 2.0;
    // BALL SIZES
    let ball_size = 10.0;
    let ball_size_apothem = ball_size / 2.0;
    // PADDLE SIZES
    let paddle_size_x = 10.0;
    let mut paddle_size_y = 150.0; // shortens over time.
    let paddle_size_x_apothem = paddle_size_x / 2.0;
    let mut paddle_size_y_apothem = paddle_size_y / 2.0;

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
        for i in 0..prec_steps {
            ball_pos_x += ball_direction_x / prec_steps as f32;
            ball_pos_y += ball_direction_y / prec_steps as f32;

            // DOES BALL HIT WALL?
            if ball_pos_y - ball_size_apothem < scr_margin {
                ball_pos_y = scr_margin + ball_size_apothem;
                ball_direction_y = ball_direction_y.abs();
                score += 1;
                play_sound_once(click);
            }
            if ball_pos_y + ball_size_apothem > screen_height() - scr_margin {
                ball_pos_y = screen_height() - scr_margin - ball_size_apothem;
                ball_direction_y = -ball_direction_y.abs();
                score += 1;
                play_sound_once(click);
            }
            if ball_pos_x + ball_size_apothem < 0.0 {
                // FOUL! Init the velocities and position
                ball_direction_x = -5.0;
                ball_direction_y = 4.0;
                ball_pos_x = screen_width() - 50.0;
                ball_pos_y = screen_height() / 2.0;
                score /= 2;
                // paddle_size_y = 150.0;
                // paddle_size_y_apothem = paddle_size_y / 2.0;
                play_sound_once(over);
            }
            if ball_pos_x + ball_size_apothem > screen_width() - scr_margin {
                ball_pos_x = screen_width() - scr_margin - ball_size_apothem;
                ball_direction_x = -ball_direction_x.abs();
                score += 1;
                play_sound_once(click);
            }

            // DOES BALL HIT PADDLE?
            if ball_pos_x - ball_size_apothem < paddle_pos_x + paddle_size_x_apothem
                && ball_pos_x + ball_size_apothem > paddle_pos_x - paddle_size_x_apothem
            {
                // In the same X.
                if ball_pos_y - ball_size_apothem < paddle_pos_y + paddle_size_y_apothem
                    && ball_pos_y + ball_size_apothem > paddle_pos_y - paddle_size_y_apothem
                {
                    // In the same Y
                    if ball_direction_x < 0.0 {
                        ball_direction_x = ball_direction_x.abs();
                        let speed = f32::sqrt(
                            ball_direction_x * ball_direction_x
                                + ball_direction_y * ball_direction_y,
                        ) * speed_mult;
                        let diff_y = (ball_pos_y - paddle_pos_y) / paddle_size_y_apothem;
                        ball_direction_y += diff_y * 5.0;
                        let speed2 = f32::sqrt(
                            ball_direction_x * ball_direction_x
                                + ball_direction_y * ball_direction_y,
                        );
                        let f = (speed / speed2).sqrt();
                        ball_direction_x *= f;
                        ball_direction_y *= f;
                        score += 1;
                        paddle_size_y /= 1.05;
                        paddle_size_y_apothem = paddle_size_y / 2.0;
                        play_sound_once(snare);
                    }
                }
            }
            let f = i as f32 / prec_steps as f32;
            let trail_color = Color::new(0.00, 0.50 * f, 1.00 * f, 1.00);
            draw_rectangle(
                ball_pos_x - ball_size_apothem,
                ball_pos_y - ball_size_apothem,
                ball_size,
                ball_size,
                trail_color,
            );
        }
        // BALL RENDERING
        let ball_color = Color::new(1.00, 0.75, 0.50, 0.50);
        draw_rectangle(
            ball_pos_x - ball_size_apothem,
            ball_pos_y - ball_size_apothem,
            ball_size,
            ball_size,
            ball_color,
        );

        // KEYBOARD INPUT - TO PADDLE
        draw_rectangle(
            paddle_pos_x - paddle_size_x_apothem,
            paddle_pos_y - paddle_size_y_apothem,
            paddle_size_x,
            paddle_size_y,
            BROWN,
        );
        let up = is_key_down(KeyCode::W);
        let down = is_key_down(KeyCode::S);
        if up {
            paddle_pos_y -= 10.0;
        }
        if down {
            paddle_pos_y += 10.0;
        }
        if paddle_pos_y - paddle_size_y_apothem < scr_margin {
            paddle_pos_y = scr_margin + paddle_size_y_apothem;
        }
        if paddle_pos_y + paddle_size_y_apothem > screen_height() - scr_margin {
            paddle_pos_y = screen_height() - scr_margin - paddle_size_y_apothem;
        }
        if paddle_size_y < 150.0 {
            paddle_size_y += 0.03;
            paddle_size_y_apothem = paddle_size_y / 2.0;
        }
        // PADDLE RENDERING
        draw_rectangle(
            paddle_pos_x - paddle_size_x_apothem,
            paddle_pos_y - paddle_size_y_apothem,
            paddle_size_x,
            paddle_size_y,
            YELLOW,
        );

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
