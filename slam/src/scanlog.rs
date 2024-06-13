
use global_lib::types::PointCloud;
use nalgebra as na;

pub fn convert_to_scans(src: &str) -> Vec<ScanLog>{
    let res: serde_json::Value = serde_json::from_str(src).unwrap();
    let mut ret = vec![];
    for entry in res.as_array().unwrap(){
        if entry.get("tag").unwrap() != "LidarScan"{
           continue;
        }

        let mut pts_transformed = vec![];
        let pts = entry.get("val").unwrap().as_object().unwrap().get("points").unwrap().as_array().unwrap();

        for pt in pts{
            let x = pt.get("x").unwrap().as_f64().unwrap() as f32;
            let y = pt.get("y").unwrap().as_f64().unwrap() as f32;
            pts_transformed.push((x,y))
        }

        ret.push(ScanLog{points: pts_transformed});
    }
    ret
}
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanLog {
    pub points: Vec<(f32, f32)>,
}

impl ScanLog {
    pub fn rotate(&mut self, rads: f32) {
        let rot = na::Rotation2::new(rads);
        for pt in &mut self.points {
            let napt = rot * na::Point2::new(pt.0, pt.1);
            *pt = (napt.x, napt.y);
        }
    }

    pub fn translate(&mut self, xt: f32, yt: f32) {
        for (x, y) in &mut self.points {
            *x += xt;
            *y += yt;
        }
    }
}

impl PointCloud for ScanLog{
    fn points(&self) -> Vec<(f32,f32)> {
        self.points.clone()
    }
}
