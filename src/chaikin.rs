use crate::point::Point;

pub fn apply_chaikin_iterations(points: &[Point], iterations: usize) -> Vec<Point> {
    let mut current_points = points.to_vec(); // creates a copy of points

    for _step in 0..iterations {
        let mut chaikin_points = Vec::new();

        chaikin_points.push(current_points[0]);

        for i in 0..current_points.len() - 1 {
            let p1 = current_points[i];
            let p2 = current_points[i + 1];

            let q = Point::new(0.75 * p1.x + 0.25 * p2.x, 0.75 * p1.y + 0.25 * p2.y);
            let r = Point::new(0.25 * p1.x + 0.75 * p2.x, 0.25 * p1.y + 0.75 * p2.y);
            chaikin_points.push(q);
            chaikin_points.push(r);
        }

        chaikin_points.push(current_points[current_points.len() - 1]);
        current_points = chaikin_points;
    }
    current_points
}
