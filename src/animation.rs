use macroquad::prelude::*;

pub fn update_animation(
    current_iteration: &mut usize,
    is_animating: &mut bool,
    last_step_time: &mut f64,
    step_duration: f64,
) {
    if *is_animating {
        let current_time = get_time();
        if current_time - *last_step_time >= step_duration {
            *current_iteration += 1;
            *last_step_time = current_time;

            if *current_iteration > 7 {
                *current_iteration = 0;
                println!("Animation...");
            }
            // else {
            //     println!("Steps: {current_iteration}");
            // }
        }
    }
}
