use macroquad::prelude::*;

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
    loop {
        clear_background(WHITE);
        draw_text("Hello, chaikin!", 20.0, 40.0, 30.0, BLACK);
        draw_text("Step 1...", 20.0, 80.0, 30.0, DARKGRAY);
        next_frame().await;
    }
}
