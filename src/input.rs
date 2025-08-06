use crate::point::Point;
use macroquad::prelude::*;

pub fn handle_input(
    points: &mut Vec<Point>,
    current_iteration: &mut usize,
    is_animating: &mut bool,
    last_step_time: &mut f64,
    warning: &mut Option<String>,
) {
    if is_mouse_button_pressed(MouseButton::Left) && !*is_animating {
        let (mouse_x, mouse_y) = mouse_position();
        let new_point = Point::new(mouse_x, mouse_y);
        points.push(new_point);
    }

    if is_key_pressed(KeyCode::C) {
        points.clear();
        *current_iteration = 0;
        *is_animating = false;
        *warning = None;
    }

    if is_key_pressed(KeyCode::Enter) && !*is_animating {
        if points.len() > 1 {
            *is_animating = true;
            *current_iteration = 0;
            *last_step_time = get_time();
            *warning = None;
        } else if points.len() == 0 {
            *warning = Some("You must place at least two points before starting.".to_string())
        } 
    }
}

pub fn should_quit() -> bool {
    is_key_pressed(KeyCode::Escape)
}
