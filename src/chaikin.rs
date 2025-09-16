use macroquad::math::Vec2;

pub fn chaikin(points: &Vec<Vec2>, iterations: usize) -> Vec<Vec2> {
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
