use std::rc::Rc;

use global_lib::types::PointCloud;
use icp_2d::ICPResult;
use nalgebra::{self as na, is_convertible};
use super::scanlog::ScanLog;
#[derive(Default)]
pub struct Slam{
    pos_abs: na::Point2<f32>,
    rot_abs: f32,

    last_translation: na::Point2<f32>,
    last_rot: f32,

    last_pose: Option<Rc<dyn PointCloud>>,
    pub min_converg: f32,
    total_converg: f64,
    included_scans: usize
}





impl Slam{
    pub fn new()->Self{
        Self{min_converg: f32::MAX,..Default::default()}
    }

    pub fn get_pos(&self) -> na::Point2<f32>{
        self.pos_abs
    }

    pub fn get_rot(&self) -> f32{
        self.rot_abs
    }

    pub fn get_avg_convergence(&self) -> f64{
        self.total_converg / self.included_scans as f64
    }

    pub fn add_pose(&mut self,scan: Rc<dyn PointCloud>){

        self.included_scans+=1;
        if self.last_pose.is_none(){
            _=self.last_pose.insert(scan.clone());
            return;
        }
        let last_pose = self.last_pose.as_ref().unwrap();
        let last_pose_points = last_pose.points();
        let icp = icp_2d::Icp::new(&last_pose_points, scan.points(),100,Unit::from_mm(1.0).to_meter(),0.01f32.to_radians(),0.01);
        let (
            ICPResult {
                mut x_offset,
                mut y_offset,
                mut rotation_offset_rad,
                convergence,
            },
            _,
        ) = icp.do_icp(self.last_translation.x, self.last_translation.y,0.0);
        if convergence < self.min_converg{
            self.min_converg = convergence;
        }
        self.total_converg += convergence as f64;

        if x_offset.abs() < Unit::from_cm(1.0).to_meter(){
            x_offset = 0.0;
        }

        if y_offset.abs() < Unit::from_cm(1.0).to_meter(){
            y_offset = 0.0;
        }


        if rotation_offset_rad.abs() < 1.0f32.to_radians(){
            rotation_offset_rad = 0.0;
        }


        self.last_translation = na::Point2::new(x_offset,y_offset);
        self.last_rot = rotation_offset_rad;

        self.rot_abs += rotation_offset_rad;
        // Normalize rotation to stay within [-π, π]
        self.rot_abs = (self.rot_abs + std::f32::consts::PI) % (2.0 * std::f32::consts::PI)
            - std::f32::consts::PI;

        // Create a rotation matrix from the updated rotation
        let rot_matrix = na::Rotation2::new(self.rot_abs);

        // Update position by applying the rotation to the offset and then adding it to the current position
        let offset = rot_matrix * na::Vector2::new(x_offset, y_offset); // Apply rotation to the offset
        self.pos_abs += na::Vector2::from(offset); // Update the position

        _=self.last_pose.insert(scan);
    }
}



struct Unit(f32);
impl Unit{
    fn from_cm(cm: f32) -> Self{
        Self(cm)
    }

    fn from_mm(mm: f32) -> Self{
        Self(mm / 10.0)
    }

    fn to_meter(&self)->f32{
        self.0 / 100.0
    }
}
