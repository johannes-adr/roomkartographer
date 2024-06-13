use crate::{distance, ramer_douglas_peucker, ScanLog};

fn angle_between(p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) -> f32 {
    let v1 = (p1.0 - p2.0, p1.1 - p2.1);
    let v2 = (p3.0 - p2.0, p3.1 - p2.1);

    let dot_product = v1.0 * v2.0 + v1.1 * v2.1;
    let magnitude_v1 = (v1.0.powi(2) + v1.1.powi(2)).sqrt();
    let magnitude_v2 = (v2.0.powi(2) + v2.1.powi(2)).sqrt();

    let cos_theta = dot_product / (magnitude_v1 * magnitude_v2);
    cos_theta.acos() * 180.0 / std::f32::consts::PI
}

fn normalize_line_length(line: &[(f32, f32)], target_length: f32) -> Vec<(f32, f32)> {
    let mut normalized_points = vec![];

    for window in line.windows(2) {
        let p1 = window[0];
        let p2 = window[1];
        let current_length = distance(p1, p2);

        if current_length != 0.0 {
            let scale = target_length / current_length;
            let new_point = (p1.0 + (p2.0 - p1.0) * scale, p1.1 + (p2.1 - p1.1) * scale);
            normalized_points.push(p1);
            normalized_points.push(new_point);
        }
    }

    normalized_points
}

fn detect_corners(points: &[(f32, f32)], angle_threshold: f32) -> Option<Vec<(f32, f32)>> {
    let mut corners = vec![];

    // Always include the first point
    corners.push(points[0]);
    let mut is_corner = false;
    for i in 1..points.len() - 1 {
        let angle = angle_between(points[i - 1], points[i], points[i + 1]);
        if (angle - 90.0).abs() <= angle_threshold {
            corners.push(points[i]);
            is_corner = true;
        }
    }
    if !is_corner {
        return None;
    }

    // Always include the last point
    corners.push(points[points.len() - 1]);

    Some(corners)
}

pub fn simplify(scan: &ScanLog) -> Vec<Vec<(f32, f32)>> {
    let mut lines: Vec<Vec<(f32, f32)>> = vec![vec![]];

    for p in scan.points.windows(2) {
        if distance(p[0], p[1]) < 0.1 {
            lines.last_mut().unwrap().push(p[0]);
        } else {
            let last = lines.last().unwrap();
            if !last.is_empty() {
                lines.push(vec![]);
            }
        }
    }

    let mut res = vec![];
    for line in lines {
        if line.len() < 50 {
            continue;
        }

        let simplified_line = ramer_douglas_peucker::ramer_douglas_peucker(line, 0.05);
        let corners = detect_corners(&simplified_line, 10.0);
        if let Some(s) = corners {
        let s =  normalize_line_length(&s, 0.5);
            res.push(s);
        }
        // res.push(simplified_line)
    }

    res
}
