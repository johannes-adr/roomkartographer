use std::f32::consts::PI;

use nalgebra as na;
use global_lib::types::ScanMeasurement;
type Point = na::Point2<f32>;
#[derive(Debug)]
pub struct World {
    pub polyg: Vec<Point>,
    spawn: Point,
}

impl Default for World {
    fn default() -> Self {
        Self::world1()
    }
}

impl World {
    pub fn world2() -> Self {
        Self::new(
            [
                (2.24,5.53), (-1.76,3.94), (1.09,3.95), (0.77,1.61), (-0.96,1.61), (-1.04,-0.67), (1.67,-0.53), (1.92,2.50), (3.43,2.52), (3.33,0.25), (5.62,3.26), (2.01,3.15), (2.14,3.86), (7.05,3.08), 
            ]
            .iter()
            .cloned(),
            // (1.5,2.0)
            (0.0,0.0)
        )
    }

    pub fn world1() -> Self {
        Self::new(
            [
                (1.05, 3.07),
                (1.65, 3.03),
                (1.59, 0.89),
                (-0.05, 0.85),
                (-0.01, -0.03),
                (3.01, 0.00),
                (3.05, 4.05),
                (1.47, 4.01),
                (-0.04, 4.00),
                (-0.01, 1.39),
                (1.07, 1.18),
                (1.05, 3.07),
            ]
            .iter()
            .cloned(),
            (0.7, 3.5)
        )
    }

    pub fn spawn(&self) -> Point {
        self.spawn
    }

    fn new(pts: impl Iterator<Item = (f32, f32)>, spawn: (f32,f32)) -> Self {
        Self {
            polyg: pts.map(|(x, y)| Point::new(x, y)).collect(),
            spawn: Point::new(spawn.0,spawn.1),
        }
    }

    fn intersect_ray_segment(
        player: na::Point2<f32>,
        angle: f32,
        p1: na::Point2<f32>,
        p2: na::Point2<f32>,
    ) -> Option<na::Point2<f32>> {
        let direction = na::Vector2::new(angle.cos(), angle.sin());

        let r = direction; // Ray's direction vector
        let s = p2 - p1; // Segment's direction vector

        let rxs = r.x * s.y - r.y * s.x; // Cross product

        // If rxs is 0, they are parallel and might be collinear
        if rxs.abs() < 1e-6 {
            return None; // For now, we'll just return no intersection if they are parallel
        }

        let q_minus_p = p1 - player;
        let t = (q_minus_p.x * s.y - q_minus_p.y * s.x) / rxs;
        let u = (q_minus_p.x * r.y - q_minus_p.y * r.x) / rxs;

        // Check if the intersection is in the correct bounds
        if t >= 0.0 && (u >= 0.0 && u <= 1.0) {
            Some(player + t * direction)
        } else {
            None
        }
    }

    pub fn do_scan(&self, target: &mut [ScanMeasurement], origin: na::Point2<f32>, mut direction_rad: f32){
        let sample_amount: usize = target.len();
        const SPREAD: f32 = 0.02;
        direction_rad -= std::f32::consts::FRAC_PI_2;
        let degree_per_sample = 360.0 / sample_amount as f32;
        for i in 0..sample_amount {
            let angle = (i as f32 * degree_per_sample).to_radians();
            if let Some(mut p) = self.ray_cast(origin, angle) {
                p.x += rand::random::<f32>() / (1.0 / SPREAD) - SPREAD / 2.0;
                p.y += rand::random::<f32>() / (1.0 / SPREAD) - SPREAD / 2.0;
                p = p.rotate(direction_rad);
                
                target[i] = p;
            }
        }
    }

    pub fn ray_cast(&self, player: Point, angle: f32) -> Option<ScanMeasurement> {
        let mut nearest_intersection: Option<ScanMeasurement> = None;
        let mut nearest_distance = f32::MAX;

        for i in 0..self.polyg.len() {
            let p1 = self.polyg[i];
            let p2 = if i + 1 == self.polyg.len() {
                self.polyg[0]
            } else {
                self.polyg[i + 1]
            };

            // Find intersection of the ray with the line segment defined by p1 and p2
            if let Some(mut intersection) = Self::intersect_ray_segment(player, angle, p1, p2) {
                let distance = na::distance(&player, &intersection);
                if distance < nearest_distance {
                    nearest_distance = distance;

                    //Offset intersection so it looks like player is origin
                    intersection.x -= player.x;
                    intersection.y -= player.y;
                    nearest_intersection = Some(ScanMeasurement::new(intersection,distance,angle));
                }
            }
        }

        nearest_intersection
    }
}