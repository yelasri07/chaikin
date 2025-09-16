use macroquad::prelude::*;

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
            let timer = get_time();

            println!("{}", timer);

            if step < 7 {
                tmp_points = chaikin(&points, step);
                step += 1;
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

fn chaikin(points: &Vec<Vec2>, iterations: usize) -> Vec<Vec2> {
    let mut result = points.clone();

    for _ in 0..iterations {
        let mut new_points = Vec::new();

        new_points.push(points[0]);

        for i in 0..result.len() - 1 {
            let p0 = result[i];
            let p1 = result[i + 1];


            let q = Vec2 {
                x: 0.75 * p0.x + 0.25 * p1.x,
                y: 0.75 * p0.y + 0.25 * p1.y,
            };

            let r = Vec2 {
                x: 0.25 * p0.x + 0.75 * p1.x,
                y: 0.25 * p0.y + 0.75 * p1.y,
            };

            new_points.push(q);
            new_points.push(r);
        }

        new_points.push(points[points.len() - 1]);

        result = new_points;
    }

    result
}
