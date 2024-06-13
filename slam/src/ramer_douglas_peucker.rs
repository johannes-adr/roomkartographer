pub fn ramer_douglas_peucker(points: Vec<(f32, f32)>, epsilon: f32) -> Vec<(f32, f32)> {
    if points.len() < 2 {
        return points;
    }

    let mut dmax = 0.0;
    let mut index = 0;
    let end = points.len();
    let start = points[0];
    let finish = points[end - 1];

    for i in 1..end - 1 {
        let d = perpendicular_distance(points[i], start, finish);
        if d > dmax {
            index = i;
            dmax = d;
        }
    }

    if dmax > epsilon {
        let mut rec_results1 = ramer_douglas_peucker(points[..=index].to_vec(), epsilon);
        let mut rec_results2 = ramer_douglas_peucker(points[index..].to_vec(), epsilon);

        rec_results1.pop(); // Remove the last point of the first part to avoid duplication
        rec_results1.append(&mut rec_results2);
        rec_results1
    } else {
        vec![start, finish]
    }
}

fn perpendicular_distance(point: (f32, f32), line_start: (f32, f32), line_end: (f32, f32)) -> f32 {
    let (x0, y0) = point;
    let (x1, y1) = line_start;
    let (x2, y2) = line_end;

    let numerator = ((y2 - y1) * x0 - (x2 - x1) * y0 + x2 * y1 - y2 * x1).abs();
    let denominator = ((y2 - y1).powi(2) + (x2 - x1).powi(2)).sqrt();

    numerator / denominator
}