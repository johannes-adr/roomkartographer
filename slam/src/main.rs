use icp_2d::ICPResult;
use na::ComplexField;
use nalgebra as na;
use serde::{Deserialize, Serialize};
use slam::Slam;
use std::time::{Duration, Instant};
use std::{fs, thread};
mod scanlog;
mod slam;
// #[derive(Encode, Decode, PartialEq, Debug)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanLog {
    pub points: Vec<(f32, f32)>,
}

impl ScanLog {
    fn rotate(&mut self, rads: f32) {
        let rot = na::Rotation2::new(rads);
        for pt in &mut self.points {
            let napt = rot * na::Point2::new(pt.0, pt.1);
            *pt = (napt.x, napt.y);
        }
    }

    fn translate(&mut self, xt: f32, yt: f32) {
        for (x, y) in &mut self.points {
            *x += xt;
            *y += yt;
        }
    }
}

use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
    let size = (800, 800);
    let window = Window::new_centered("scans", size).unwrap();

    window.run_loop(MyWindowHandler::new(size));
}

#[derive(Default)]
struct MyWindowHandler {
    scans: Vec<ScanLog>,
    size: (u32, u32),
    i: usize,
    slam: Slam
}

impl MyWindowHandler {
    fn new(size: (u32, u32)) -> Self {
        let scans: Vec<ScanLog> =
            bincode::deserialize(&fs::read("./scanlog_simulation.binary").unwrap()).unwrap();
        MyWindowHandler {
            scans,
            size,
            slam: Slam::new(),
            ..Default::default()
        }
    }

    fn transform_coordinate(&self, coord: f32, x_axis: bool) -> f32 {
        let zoom_fac = 1.0;
        let range = 12.0 * zoom_fac;
        let mid_point = range / 2.0;

        let canvsize = if x_axis { self.size.0 } else { self.size.1 } as f32;

        return ((coord + mid_point) / range) * canvsize;
    }
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        if self.i == 0 {
            graphics.clear_screen(Color::from_rgb(0.0, 0.0, 0.0));
        }


        let mut scan = self.scans[self.i].clone();
        self.i += 1;

        self.slam.add_pose(scan.clone());
        let pos = self.slam.get_pos();
        let rot = self.slam.get_rot();

       
        // draw_scan(self, &scan2, graphics,Color::GREEN);
      
        {
            let x = self.transform_coordinate(pos.x, true);
            let y = self.transform_coordinate(pos.y, false);
            graphics.draw_circle((x, y), 3.0, Color::RED);
        }
        scan.rotate(rot);

        scan.translate(pos.x, pos.y);

        // draw_scan(self, &scan, graphics,Color::RED);
        draw_scan(self, &scan, graphics, Color::BLUE);

        // for (x,y) in scan2.points.iter().chain(firstscan.points.iter()){
        //     let x = self.transform_coordinate(*x, true);
        //     let y = self.transform_coordinate(*y, false);
        //     graphics.draw_circle((x, y), 2.0, Color::BLUE);
        // }
        thread::sleep(Duration::from_millis(20));
        helper.request_redraw();
    }
}

fn draw_scan(
    this: &MyWindowHandler,
    scan: &ScanLog,
    graphics: &mut Graphics2D,
    color: Color,
) {
    for (x, y) in &scan.points {
        let x = this.transform_coordinate(*x, true);
        let y = this.transform_coordinate(*y, false);
        graphics.draw_circle((x, y), 2.0, color);
    }
}

// fn main(){
//     let start = Instant::now();
//     let scans = scanlog::convert_to_scans(&std::fs::read_to_string("./SLAMData-4.json").unwrap());

//     let res = bincode::serialize(&scans).unwrap();

//     fs::write("./scanlog-4.binary", res);
//     println!("Took {:?}",start.elapsed());
// }

// fn main() {
//     let scans: Vec<ScanLog> = bincode::deserialize(&fs::read("./scanlog.binary").unwrap()).unwrap();

//    scans.windows(2).for_each(|s|{
//     let first = &s[0];
//     let second = s[1].clone();
//     let mut icp = icp_2d::Icp::new_default(&first.points, second.points);
//     let (res,_) = icp.do_icp(0.0, 0.0, 0.0);
//     let ICPResult{x_offset: x,y_offset: y,rotation_offset_rad: rot,convergence} = res;
//     println!("{} => {x} {y} {}",convergence,rot.to_degrees());
//    });
// }

fn txtlog_to_binary() {
    let mut scans = read_scan("out1");
    scans.append(&mut read_scan("out2"));
    scans.append(&mut read_scan("out3"));
    let res = bincode::serialize(&scans).unwrap();

    fs::write("./scanlog.binary", res);
}

fn read_scan(dir: &str) -> Vec<ScanLog> {
    let mut i = 0;
    let mut scans = vec![];
    loop {
        if let Ok(s) = fs::read_to_string(format!("{dir}/scan{i}.txt")) {
            let scan = ScanLog {
                points: parse_scan(&s),
            };
            scans.push(scan);
            i += 1;
        } else {
            break;
        }
    }
    return scans;
}

fn parse_scan(s: &str) -> Vec<(f32, f32)> {
    let scan = s
        .split("\n")
        .map(|line| {
            let mut line = line.split_whitespace().map(|s| s.parse::<f32>().unwrap());
            (line.next().unwrap(), line.next().unwrap())
        })
        .collect::<Vec<_>>();
    return scan;
}


