use macroquad::prelude::*;

mod animation;
mod chaikin;
mod input;
mod point;
mod ui;

use point::Point;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chaikin".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut click_positions: Vec<Point> = Vec::new();
    let mut current_iteration = 0;
    let mut is_animating = false;
    let mut last_step_time = 0.0;
    let step_duration = 1.0;

    loop {
        input::handle_input(
            &mut click_positions,
            &mut current_iteration,
            &mut is_animating,
            &mut last_step_time,
        );

        if input::should_quit() {
            break;
        }

        animation::update_animation(
            &mut current_iteration,
            &mut is_animating,
            &mut last_step_time,
            step_duration,
        );

        clear_background(WHITE);

        ui::draw_ui(&click_positions, current_iteration, is_animating);
        ui::draw_points(&click_positions);
        ui::draw_animation(&click_positions, current_iteration, is_animating);

        next_frame().await;
    }
}
