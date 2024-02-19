use icp_2d::ICPPoint;
use kdtree::{distance::squared_euclidean, KdTree};
use nalgebra as na;
use rplidar_drv::ScanPoint;
use std::{
    fs,
    ops::{Add, Deref, DerefMut},
    time::{Instant, SystemTime}, cell::Cell,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ScanMeasurement {
    point: na::Point2<f32>,
    distance: f32,
    angle: f32,
}

impl ICPPoint for ScanMeasurement{
    fn point(&self) -> na::Point2<f32> {
        self.point
    }

    fn is_data_valid(&self) -> bool {
        true
    }

    fn translate(&self,x: f32,y: f32) -> Self {
        unimplemented!()
    }

    fn rotate(&self,angle_rad: f32) -> Self {
        unimplemented!()
    }
}

pub fn parse_scan(name: &str) -> Scan {
    let str = fs::read_to_string(name).unwrap();
    let read = str.split("\n").map(|line| {
        let mut split = line.split_whitespace();
        let p: na::Point2<f32> = na::Point2::new(
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        );

        ScanMeasurement::from_point(p)
    });

    Scan::new_now(read.collect::<Box<_>>())
}

impl ScanMeasurement {
    pub fn new(point: na::Point2<f32>, distance: f32, angle: f32) -> Self {
        let mut measurement = Self {
            point,
            distance,
            angle,
        };
        measurement.normalize_angle();
        debug_assert!(measurement.is_data_valid(), "{measurement:?}");
        return measurement;
    }

    pub fn from_point(p: na::Point2<f32>) -> Self {
        let dist = na::distance(&p, &na::Point2::new(0.0, 0.0));
        let radians = f32::atan2(p.y, p.x);
        let measurement = ScanMeasurement::new(p, dist, radians);
        debug_assert!(measurement.is_data_valid(), "{measurement:?}");
        return measurement;
    }

    pub fn from_polar(distance: f32, angle: f32) -> Self {
        let x = angle.cos() * distance;
        let y = angle.sin() * distance;
        let measurement = ScanMeasurement::new(na::Point2::new(x, y), distance, angle);
        debug_assert!(measurement.is_data_valid(), "{measurement:?}");
        measurement
    }

    pub fn point(&self) -> na::Point2<f32> {
        self.point
    }

    pub fn translate(&self, x: f32, y: f32) -> ScanMeasurement {
        Self::from_point(na::Point2::new(self.x + x, self.y + y))
    }

    pub fn rotate(&self, angle_rad: f32) -> ScanMeasurement {
        Self::from_polar(self.distance, self.angle + angle_rad)
    }

    fn normalize_angle(&mut self) {
        const PI2: f32 = 2.0 * std::f32::consts::PI;
        self.angle %= PI2;
        if self.angle < 0.0 {
            self.angle += PI2;
        }
        debug_assert!(self.angle >= 0.0 && self.angle <= PI2);
    }

    pub fn distance_to_origin(&self) -> f32 {
        self.distance
    }
    ///radians
    pub fn angle_from_origin(&self) -> f32 {
        self.angle
    }

    fn is_data_valid(&self) -> bool {
        // Calculate the x and y coordinates based on the provided angle and distance
        let calculated_x = self.angle.cos() * self.distance;
        let calculated_y = self.angle.sin() * self.distance;
        // println!("{calculated_x} {calculated_y}, {}",self.point());
        points_equal_tolerance(na::Point2::new(calculated_x, calculated_y), self.point())
    }
}

impl Deref for ScanMeasurement {
    type Target = na::Point2<f32>;

    fn deref(&self) -> &Self::Target {
        &self.point
    }
}

impl DerefMut for ScanMeasurement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.point
    }
}

impl From<Vec<ScanPoint>> for Scan {
    fn from(value: Vec<ScanPoint>) -> Self {
        let time = SystemTime::now();

        let ret: Vec<ScanMeasurement> = value
            .iter()
            .map(|sp| {
                let angle = sp.angle();
                let distance = sp.distance();

                let x = distance * angle.cos();
                let y = distance * angle.sin();

                ScanMeasurement {
                    point: na::Point2::new(x, y),
                    distance,
                    angle,
                }
            })
            .filter(|p| !(p.x == 0.0 && p.y == 0.0))
            .collect();
        return Scan::new(ret, time);
    }
}

#[derive(Debug, Clone)]
pub struct Scan {
    points: Box<[ScanMeasurement]>,
    center_of_mass: Option<na::Point2<f32>>,
    kd_tree: Option<KdTree<f32, usize, [f32; 2]>>,
    time: SystemTime,
}

impl Default for Scan{
    fn default() -> Self {
        Self { points: Default::default(), center_of_mass: Default::default(), kd_tree: Default::default(), time: SystemTime::now() }
    }
}

impl Scan {
    pub fn new_now(points: impl Into<Box<[ScanMeasurement]>>) -> Self {
        Self::new(points, SystemTime::now())
    }

    pub fn new(points: impl Into<Box<[ScanMeasurement]>>, time: SystemTime) -> Self {
        Self {
            points: points.into(),
            time,
            center_of_mass: Default::default(),
            kd_tree: Default::default(), // kd_tree: KdTree::new(2),
        }
    }

    //Calculated lazy since not every scan might need a center of mass
    pub fn get_center_of_mass(&mut self) -> na::Point2<f32> {
        if let Some(s) = self.center_of_mass{
            return s;
        }
        *self.center_of_mass.insert(self.calculate_center_of_mass())
    }

    pub fn get_center_of_mass_mut(&mut self) -> &mut na::Point2<f32> {
        if self.center_of_mass.is_some(){
            self.center_of_mass.as_mut().unwrap()
        }else{
            self.center_of_mass.insert(self.calculate_center_of_mass())
        }
    }

    pub fn closest_point(&self, point: na::Point2<f32>) -> na::Point2<f32> {
        let mut ptsiter = self.points.iter();
        let mut closest_pt = ptsiter.next().unwrap().point();
        let mut closes_dist = na::distance_squared(&point, &closest_pt);
        for pt in ptsiter {
            let dist = na::distance_squared(pt, &point);
            if dist < closes_dist {
                closes_dist = dist;
                closest_pt = pt.point();
            }
        }
        closest_pt
    }

    pub fn closest_point_kd(&mut self, point: na::Point2<f32>) -> na::Point2<f32> {
        let kd_tree = if let Some(s) = &self.kd_tree {
            s
        } else {
            let mut kd_tree = KdTree::new(2);
            self.points
                .iter()
                .enumerate()
                .for_each(|(i, p)| kd_tree.add([p.x, p.y], i).unwrap());
            self.kd_tree.insert(kd_tree)
        };

        let pt = kd_tree
            .iter_nearest(&[point.x, point.y], &squared_euclidean)
            .unwrap()
            .next()
            .unwrap();

        self.points[*pt.1].point()
    }

    fn calculate_center_of_mass(&self) -> na::Point2<f32> {
        let mut center_of_mass = na::Point2::new(0.0, 0.0);
        self.points
            .iter()
            .for_each(|p| center_of_mass += p.point().coords);
        center_of_mass /= self.points.len() as f32;
        return center_of_mass;
    }

    pub fn get_points(&self) -> &[ScanMeasurement] {
        &self.points
    }

    pub fn translate(&mut self, x: f32, y: f32) -> &mut Self {
        assert!(self.kd_tree.is_none());
        self.points.iter_mut().for_each(|p| {
            //v.x += val;
            *p = p.translate(x, y)
        });
        if let Some(s) = &mut self.center_of_mass {
            *s += na::Point2::new(x, y).coords;
            debug_assert!(points_equal_tolerance(*s, self.calculate_center_of_mass()));
        };
        self
    }

    pub fn rotate(&mut self, rot_rad: f32) -> &mut Self {
        assert!(self.kd_tree.is_none());
        self.points.iter_mut().for_each(|p| {
            *p = p.rotate(rot_rad);
        });
        self.center_of_mass = None;

        // *center_of_mass = ScanMeasurement::from_point(*center_of_mass).rotate(rot_rad).point;
        // todo!("Rotate center of mass");
        debug_assert!(points_equal_tolerance(
            self.get_center_of_mass(),
            self.calculate_center_of_mass()
        ));
        self
    }

    pub fn assert_valid(&self) {
        self.points.iter().for_each(|p| assert!(p.is_data_valid()));
        if let Some(s) = self.center_of_mass {
            assert!(points_equal_tolerance(s, self.calculate_center_of_mass()));
        }
    }

    pub fn get_time(&self) -> &SystemTime {
        &self.time
    }
}

fn points_equal_tolerance(p1: na::Point2<f32>, p2: na::Point2<f32>) -> bool {
    fn floats_are_equal(a: f32, b: f32) -> bool {
        (a - b).abs() <= 0.001
    }
    return floats_are_equal(p1.x, p2.x) && floats_are_equal(p1.y, p2.y);
}
