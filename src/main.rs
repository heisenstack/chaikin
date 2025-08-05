use macroquad::prelude::*;
#[derive(Clone, Copy, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Point{x, y}
    }
}
fn window_conf() -> Conf {
    Conf {
        window_title: "Chaikin".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]

async fn main() {
    let mut click_position: Vec<Point> = Vec::new();

    loop {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let new_point = Point::new(mouse_x, mouse_y);
            click_position.push(new_point.clone());
            println!("Mouse clicked at: ({:?})", new_point);
        }

        clear_background(WHITE);
        draw_text("Hello, chaikin!", 20.0, 40.0, 30.0, BLACK);
        draw_text(
            &format!("Points placed: {}", click_position.len()),
            20.0,
            70.0,
            30.0,
            BLACK,
        );
        for point in &click_position {
            draw_circle(point.x, point.y, 8.0, BLUE);
            draw_circle_lines(point.x, point.y, 8.0, 2.0, DARKBLUE);
        }

        let (current_x, current_y) = mouse_position();
        draw_text(
            format!("Mouse: ({}, {})", current_x, current_y).as_str(),
            20.0,
            100.0,
            30.0,
            BLACK,
        );
        next_frame().await;
    }
}
