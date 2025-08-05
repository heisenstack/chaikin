use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Chaikin Step 5 - Escape to Quit".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut click_positions: Vec<Point> = Vec::new();

    loop {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let new_point = Point::new(mouse_x, mouse_y);
            click_positions.push(new_point);
            println!("Clicked at: {:?}", new_point);
        }

        if is_key_pressed(KeyCode::C) {
            click_positions.clear();
            // println!("{}", click_positions.len());
        }

        if is_key_pressed(KeyCode::Escape) {
            // println!("Exit..!");
            break;
        }

        clear_background(WHITE);

        draw_text("Click anywhere to place points!", 20.0, 40.0, 24.0, BLACK);
        draw_text("Press 'C' to clear all points", 20.0, 70.0, 20.0, BLUE);
        draw_text("Press 'ESC' to quit", 20.0, 100.0, 20.0, BLUE);
        draw_text(
            &format!("Points placed: {}", click_positions.len()),
            20.0,
            130.0,
            20.0,
            DARKGRAY,
        );

        for point in &click_positions {
            draw_circle(point.x, point.y, 8.0, BLUE);
            draw_circle_lines(point.x, point.y, 8.0, 2.0, DARKBLUE);
        }

        let (current_x, current_y) = mouse_position();
        draw_text(
            &format!("Mouse: ({:.0}, {:.0})", current_x, current_y),
            20.0,
            160.0,
            16.0,
            GRAY,
        );

        next_frame().await;
    }
}
