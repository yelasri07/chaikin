use macroquad::prelude::*;
mod chaikin;
use chaikin::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Fixed Window".to_owned(),
        window_width: 500,
        window_height: 700,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut points: Vec<Vec2> = Vec::new();
    let mut tmp_points: Vec<Vec2> = Vec::new();
    let mut started = false;
    let mut step = 0;
    let mut timer = get_time();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Enter) {
            started = true
        }

        if is_mouse_button_pressed(MouseButton::Left) && !started {
            let (x, y) = mouse_position();
            points.push(Vec2 { x: x, y: y });
        }

        for point in points.iter() {
            draw_circle(point.x, point.y, 5.0, Color::from_rgba(155, 200, 35, 255));
        }

        if started {
            if step < 7 && get_time() > timer + 0.5 {
                tmp_points = chaikin(&points, step);
                step += 1;
                timer = get_time();
            }

            for i in 0..tmp_points.len() - 1 {
                let p1 = tmp_points[i];
                let p2 = tmp_points[i + 1];
                draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, YELLOW);
            }
        }

        next_frame().await;
    }
}
