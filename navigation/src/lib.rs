use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::{Arc};
use std::thread;
use std::time::{Duration, Instant};

use global_lib::bus::{BusLidarScan, BusRoverMovement};
use global_lib::types::{PointCloud, Scan};
use line_drawing::Bresenham;
use nalgebra::Point2;
use slam::Slam;
use speedy2d::color::Color;
use speedy2d::shape::Rectangle;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
mod world;

use pathfinding::prelude::{astar, bfs, bfs_reach, dfs};
use tokio::sync::broadcast::Receiver;
use tokio::sync::RwLock;
use world::{ChunkCell, GlobalCellIndex, PointInt, World,Chunk};

pub fn start_navigator(movement_bus: BusRoverMovement, lidar_bus: BusLidarScan){
    let size = (800, 800);
    let window = Window::new_centered("Robot", size).unwrap();
    let handler = NavigationWindowHandler::new(size, movement_bus, lidar_bus);

    window.run_loop(handler);
}



struct NavigationWindowHandler{
    size: (u32,u32),
    world: World,
    slam: Slam,
    movement_bus: BusRoverMovement,
    lidar_bus: Receiver<Arc<Scan>>,
    pub path_finding_target: Option<GlobalCellIndex>,
    path_to_target: VecDeque<GlobalCellIndex>,
}





impl WindowHandler for NavigationWindowHandler{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        // thread::sleep(Duration::from_millis(1000));
        graphics.clear_screen(Color::from_rgb(1.0, 1.0, 1.0));
        while let Ok(scan) = self.lidar_bus.try_recv(){
            self.insert_scan_in_world(scan);
        }

        for chunk in self.world.get_chunks(){
            chunk.iterate(|p,(w,h),c|{
                let color = match c{
                    ChunkCell::Closed=>Color::BLACK,
                    ChunkCell::Open=>Color::LIGHT_GRAY,
                    ChunkCell::CloseToClose=>Color::from_rgb(0.65, 0.65, 0.65),
                    ChunkCell::Unknown => Color::from_rgb(0.95, 0.95, 0.95),
                    _=>Color::WHITE
                };

                graphics.draw_rectangle(Rectangle::from_tuples(self.transform_point((p.x,p.y)), self.transform_point((p.x+w,p.y+h))), color);
            });
        }


        for p in self.path_to_target.iter().cloned().map(GlobalCellIndex::into_point){
             graphics.draw_circle(self.transform_point((p.x,p.y)), 2.0, Color::GREEN);
        }

        let p = self.slam.get_pos();
        graphics.draw_circle(self.transform_point((p.x,p.y)), 8.0, Color::RED);

        helper.request_redraw();
        self.navigate();
    }
}




impl NavigationWindowHandler{

    fn navigate(&mut self){
        self.do_pathfinding(self.slam.get_pos());
        if let Some(mut next_point) = self.path_to_target.pop_front().map(GlobalCellIndex::into_point){
            println!("{:?}, {}",next_point, self.slam.get_pos());
            let x = next_point.x;
            let y = next_point.y * -1.0;

            _=self.movement_bus.send(global_lib::types::RoverMovement::Teleport(nalgebra::Point2::new(y,x)));
        }else{
            println!("Done");
        }
    }


    fn new(size: (u32,u32),movement_bus: BusRoverMovement, lidar_bus: BusLidarScan) -> Self{
        let res = lidar_bus.subscribe();
        Self{size,world: World::default(),movement_bus,lidar_bus: lidar_bus.subscribe(), slam: Slam::new(), path_to_target: Default::default(), path_finding_target: Default::default()}
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

    fn insert_scan_in_world(&mut self,scan: Arc<Scan>){
        let mut scan = (*scan).clone();
        let scan_slam: Rc<dyn PointCloud> = Rc::new(scan.clone());

        self.slam.add_pose(scan_slam.into());
        let pos = self.slam.get_pos();
        let rot = self.slam.get_rot();
        scan.rotate(rot);
        scan.translate(pos.x, pos.y);

        for pt in scan.get_points(){
            let start = GlobalCellIndex::from_point(pos);
            let end =  GlobalCellIndex::from_point(**pt);

            let line = Bresenham::new(
                (start.x,start.y),
                (end.x,end.y)
            )
            .map(|c| GlobalCellIndex(PointInt::new(c.0, c.1)));
            for pos in line {
                let cell = self.world.get_cell_mut(pos);
                if cell.get() == ChunkCell::Unknown {
                    cell.set(ChunkCell::Open);
                }
            }

            let p = GlobalCellIndex::from_point(**pt);
            self.world.get_cell_mut(p).set(ChunkCell::Closed);
        }


        scan.get_points().iter().map(|p|**p).for_each(|p| {
            for (c, _) in GlobalCellIndex::from_point(p).neighbours() {
                let cell = self.world.get_cell_mut(c);
                cell.set(ChunkCell::Closed);
            }

            for c in GlobalCellIndex::from_point(p).radius(12) {
                let cell = self.world.get_cell_mut(c);
                if cell.get() == ChunkCell::Open {
                    cell.set(ChunkCell::CloseToClose);
                }
            }
        });
    }

    fn find_last_unknown_wall_bfs(&self, start: GlobalCellIndex) -> Option<GlobalCellIndex> {
        let possible_nodes = |this: &GlobalCellIndex| {
            this.neighbours().into_iter().filter_map(|(n, _)| {
                let cell = self.world.get_cell(n);
                if cell == ChunkCell::Open
                    || cell == ChunkCell::CloseToClose
                    || cell == ChunkCell::Unknown
                {
                    Some(n)
                } else {
                    None
                }
            })
        };

        let goal = |cell: &GlobalCellIndex| {
            // let surrounding_walls = cell.neighbours().iter().filter(|(pos,_)|self.room_scanned.get_cell(*pos) == ChunkCell::Unknown).count();
            // surrounding_walls == 1

            self.world.get_cell(*cell) == ChunkCell::Unknown
        };
        let binding = bfs(&start, possible_nodes, goal)?;
        binding.last().as_ref().cloned().cloned()
    }

    pub fn do_pathfinding(&mut self, position: Point2<f32>) {
        let start = GlobalCellIndex::from_point(position);

        let goal = if let Some(s) = self.find_last_unknown_wall_bfs(start) {
            s
        } else {
            self.path_to_target = Default::default();
            return;
        };
        self.path_finding_target = Some(goal);


        let possible_nodes = |this: &GlobalCellIndex| {
            this.neighbours().into_iter().filter_map(|(n, direction)| {
                let cell = self.world.get_cell(n);
                let open = cell == ChunkCell::Open;
                let close_to_wall = cell == ChunkCell::CloseToClose;

                if open || close_to_wall {
                    let mut cost = if direction.is_diagonal() { 10 } else { 1 };
                    if close_to_wall {
                        cost += 100;
                    }
                    Some((n, cost))
                } else {
                    None
                }
            })
        };

        let success = |cell: &GlobalCellIndex| {
            let surrounding_walls = cell
                .neighbours()
                .iter()
                .filter(|(pos, _)| self.world.get_cell(*pos) == ChunkCell::Unknown)
                .count();
            surrounding_walls > 0
        };

        let path_to_goal = astar(
            &start,
            possible_nodes,
            |p| p.manhatten_distance(&goal),
            success,
        );
        if let Some((path_to_goal, _)) = path_to_goal {
            // println!("{:?} - Took: {:?}",path_to_goal.last().unwrap().into_point(), start_time.elapsed());

            self.path_to_target = path_to_goal.into();
            self.path_to_target.pop_front();
        } else {
            println!("Error");
            self.path_to_target = Default::default();
        }
    }

}
