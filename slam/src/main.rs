use global_lib::types::PointCloud;
use itertools::Itertools;
use nalgebra as na;
use point_simpifier::simplify;
use serde::{Deserialize, Serialize};
use slam::Slam;
use std::rc::Rc;
use std::time::{Duration, Instant};
use std::{fs, thread};
mod scanlog;
use scanlog::ScanLog;
mod ramer_douglas_peucker;
mod point_simpifier;
// #[derive(Encode, Decode, PartialEq, Debug)]


use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
    let size = (800, 800);
    let window = Window::new_centered("scans", size).unwrap();
    let mut wndhandler = MyWindowHandler::new(size);
    let start = Instant::now();
    // wndhandler.do_slam();
    println!("Took: {:?}",start.elapsed());
    window.run_loop(wndhandler);
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

#[derive(Default)]
struct MyWindowHandler {
    scans: Vec<ScanLog>,
    size: (u32, u32),
    i: usize,
    slam: Slam,
}

impl MyWindowHandler {
    fn new(size: (u32, u32)) -> Self {
        let mut scans: Vec<ScanLog> =
            bincode::deserialize(&fs::read("./scanlog-4.binary").unwrap()).unwrap(); //"./scanlog_simulation.binary". ./scanlog-4.binary

        println!("{}",scans.len());
        MyWindowHandler {
            scans,
            size,
            slam: Slam::new(),
            i: 0,
        }
    }

    fn transform_coordinate(&self, coord: f32, x_axis: bool) -> f32 {
        let zoom_fac = 1.0;
        let range = 12.0 * zoom_fac;
        let mid_point = range / 2.0;

        let canvsize = if x_axis { self.size.0 } else { self.size.1 } as f32;

        return ((coord + mid_point) / range) * canvsize;
    }

    fn transform_point(&self, coord: (f32, f32)) -> (f32, f32) {
        (
            self.transform_coordinate(coord.0, true),
            self.transform_coordinate(coord.1, false),
        )
    }

    fn do_slam(&mut self){
        let mut min = Duration::from_secs(10000);
        let mut max = Duration::from_secs(0);
        let mut avg = Duration::from_secs(0);
        for mut scan in self.scans.clone(){
            let start = Instant::now();
            let slam_scan: Rc<dyn PointCloud> = Rc::new(scan.clone());
            self.slam.add_pose(slam_scan);


            let pos = self.slam.get_pos();
            let rot = self.slam.get_rot();


            {
                let x = self.transform_coordinate(pos.x, true);
                let y = self.transform_coordinate(pos.y, false);
            }
            scan.rotate(rot);

            scan.translate(pos.x, pos.y);
            let elapsed = start.elapsed();

            avg += elapsed;
            if elapsed < min{
                min = elapsed;
            }
            if elapsed > max{
                max = elapsed
            }
        }

        println!("Min {:?}, Max {:?}, Avg {:?}", min,max,avg.div_f32(self.scans.len() as f32));
        println!("AVG Conv: {}, Min Conv: {}",self.slam.get_avg_convergence(), self.slam.min_converg);
    }

}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        if self.i > 240{
            thread::sleep(Duration::from_millis(100));
            // helper.request_redraw();
            // return;
        }

        if self.i == 0 || true {
            // self.i = 60;
            graphics.clear_screen(Color::from_rgb(1.0, 1.0, 1.0));
        }
        if self.i >= self.scans.len() {
            helper.request_redraw();
            return;
        }

        let mut scan = self.scans[self.i].clone();
        // scan.points = simplify(&scan).into_iter().flatten().collect();
        self.i += 1;

        let slam_scan: Rc<dyn PointCloud> = Rc::new(scan.clone());
        self.slam.add_pose(slam_scan);

        let pos = self.slam.get_pos();
        let rot = self.slam.get_rot();
        println!("{}, {}",pos,rot);
        // draw_scan(self, &scan2, graphics,Color::GREEN);

        {
            let x = self.transform_coordinate(pos.x, true);
            let y = self.transform_coordinate(pos.y, false);
            graphics.draw_circle((x, y), 3.0, Color::RED);
        }
        // scan.rotate(rot);

        // scan.translate(pos.x, pos.y);

        if self.i > 0 {
            draw_scan(self, &scan, graphics, Color::BLACK)
        }

        // draw_scan(self, &scan, graphics,Color::RED);
        // draw_scan(self, &scan, graphics, Color::BLUE);

        // for (x,y) in scan2.points.iter().chain(firstscan.points.iter()){
        //     let x = self.transform_coordinate(*x, true);
        //     let y = self.transform_coordinate(*y, false);
        //     graphics.draw_circle((x, y), 2.0, Color::BLUE);
        // }
        //


        helper.request_redraw();
    }
}

fn draw_scan(this: &MyWindowHandler, scan: &ScanLog, graphics: &mut Graphics2D, color: Color) {
    for (x, y) in &scan.points {
        let x = this.transform_coordinate(*x, true);
        let y = this.transform_coordinate(*y, false);
        graphics.draw_circle((x, y), 2.0, color);
    }
    return;
    let lines = simplify(scan);
    for line in lines{
        if line.len() < 2{
            continue;
        }
        for p in line.windows(2){
            graphics.draw_circle(this.transform_point(p[0]), 4.0, Color::GREEN);
            graphics.draw_line(this.transform_point(p[0]), this.transform_point(p[1]), 2.0, Color::GREEN)
        }

        graphics.draw_circle(this.transform_point(*line.last().unwrap()), 4.0, Color::GREEN);
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

pub fn distance(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    (dx * dx + dy * dy).sqrt()
}
