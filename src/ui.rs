use crate::chaikin;
use crate::point::Point;
use macroquad::prelude::*;

pub fn draw_ui(points: &[Point], current_iteration: usize, is_animating: bool) {
    draw_text("Click anywhere to place points!", 20.0, 20.0, 24.0, BLACK);
    draw_text("Press 'ENTER' to start animation", 20.0, 50.0, 20.0, GREEN);
    draw_text("Press 'C' to clear all points", 20.0, 560.0, 20.0, BLUE);
    draw_text("Press 'ESC' to quit", 20.0, 590.0, 20.0, RED);
    draw_text(
        &format!("Points placed: {}", points.len()),
        20.0,
        80.0,
        20.0,
        DARKGRAY,
    );

    if is_animating {
        draw_text(
            &format!("Animation - Step: {}/7", current_iteration),
            20.0,
            110.0,
            20.0,
            RED,
        );
    } else {
        draw_text(
            "Put 2 dots or more, enter \"ENTER\" to start Animation",
            20.0,
            110.0,
            20.0,
            DARKGRAY,
        );
    }

    let (current_x, current_y) = mouse_position();
    draw_text(
        &format!("Mouse: ({:.0}, {:.0})", current_x, current_y),
        670.0,
        590.0,
        16.0,
        GRAY,
    );
}

pub fn draw_points(points: &[Point]) {
    for point in points {
        draw_circle(point.x, point.y, 8.0, BLUE);
        draw_circle_lines(point.x, point.y, 8.0, 2.0, DARKBLUE);
    }
}

pub fn draw_animation(points: &[Point], current_iteration: usize, is_animating: bool) {
    if is_animating && points.len() > 1 {
        if current_iteration == 0 {
            for i in 0..points.len() - 1 {
                let start = points[i];
                let end = points[i + 1];
                draw_line(start.x, start.y, end.x, end.y, 2.0, GRAY);
            }
        } else {
            let current_points = chaikin::apply_chaikin_iterations(points, current_iteration);

            for i in 0..current_points.len() - 1 {
                let start = current_points[i];
                let end = current_points[i + 1];
                draw_line(start.x, start.y, end.x, end.y, 2.0, GRAY);
            }
        }
    }
}
