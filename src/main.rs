use macroquad::prelude::*;

struct Point {
    x: f32,
    y: f32,
}

// impl Point {
//     fn new(x: f32, y: f32) -> Self {
//         Point{x, y}
//     }
// }
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
    let mut click_position: Vec<(f32, f32)> = Vec::new();
    loop {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            click_position.push((mouse_x, mouse_y));
            println!("Mouse clicked at: ({}, {})", mouse_x, mouse_y);
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
        for (x, y) in &click_position {
            draw_circle(*x, *y, 8.0, BLUE);
            draw_circle_lines(*x, *y, 8.0, 2.0,     DARKBLUE);
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
