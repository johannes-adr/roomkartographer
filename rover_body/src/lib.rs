use std::{fs, sync::Arc, thread, time::{SystemTime, self, Duration, Instant}};

use global_lib::{bus::{BusIMURotation, BusRoverMovement, BusLidarScan}, types::Scan};
use icp_2d::{Icp, ICPResult};
use log::info;
use tokio::sync::RwLock;
use nalgebra as na;
mod rover_body;

pub fn start_rover_body_and_imu(movement_bus: BusRoverMovement, imu_bus: BusIMURotation, lidar_bus: BusLidarScan) {
    {
        let movement_bus = movement_bus.clone();
        tokio::task::spawn(async move {
            let gpio = rppal::gpio::Gpio::new().unwrap();
            let mut body = rover_body::RoverBody::new(&gpio, movement_bus);
        
            body.listen_bus().await;
        });
    }


    let mut controller = rover_body::imu::ImuController::new();
    let location_ref = controller.get_location();
    // tokio::spawn(async move{
    //     controller.publish_data_loop(imu_bus).await;
    // });

    let current_scan = Arc::new(RwLock::new(Arc::new(Scan::default())));
    {
        let current_scan = current_scan.clone();
        tokio::spawn(async move{
            loop{
                let scan = lidar_bus.subscribe().recv().await.unwrap();
                *current_scan.write().await = scan;
            }
        });
    }


    /*
    This is the 
    */
    tokio::task::spawn(async move{
        let mut last_scan: Arc<Scan> = Default::default();
        let mut last_rotation: f32 = 0.0;
        let mut i = 0;
        loop{

            let rw_lock_scan = current_scan.read().await;
            let scan = rw_lock_scan.clone();
            drop(rw_lock_scan);
      
            


            if last_scan.get_points().len() == 0{
                last_scan = scan;
                continue;
            }
           
            if scan.get_time() == last_scan.get_time(){
                tokio::time::sleep(Duration::from_millis(30)).await;
                continue;
            }
            
            let location = location_ref.read().await;
            let rotation_diff = location.rotation - last_rotation;
            last_rotation = location.rotation;
            drop(location);


            let icp_points_2 = scan.get_points().iter().map(|p|p.point()).collect();
            let start = Instant::now();
            let (ICPResult{x_offset:x,y_offset:y,rotation_offset_rad:icp_rot,convergence},_) = tokio::task::spawn_blocking(move ||{
                let last_scan = last_scan.clone();
                let icp = Icp::new_default(last_scan.get_points(), icp_points_2);
                icp.do_icp(0.0, 0.0,0.0 /*-rotation_diff*/)
            }).await.unwrap();
            info!("ICP Took {:?} - Convergence: {:.2}", start.elapsed(), convergence * 100.0);
            if convergence < 0.2{
                movement_bus.send(global_lib::types::RoverMovement::Stop);
            }
            let mut location = location_ref.write().await;
            
            // Update the rotation first
            if icp_rot > 1.0f32.to_radians(){
                location.rotation = last_rotation + icp_rot;
            }

            // Now calculate the displacement in global coordinates
            let mut displacement = na::Vector2::new(0.0, 0.0);
            if x > 0.01{
                displacement.x = x;
            }
            if y > 0.01{
                displacement.y = y;
            }

            // if displacement.x != 0.0 || displacement.y != 0.0{
            //     let txt = scan.get_points().iter().map(|p|format!("{} {}",p.x,p.y)).collect::<Vec<_>>().join("\n");
            //     fs::write(format!("./out/scan{i}.txt"), txt);
            //     i+=1;
            // }
                
            // Rotate the displacement vector by the rover's current rotation
            let rotation_matrix = na::Rotation2::new(location.rotation);
            let rotated_displacement = rotation_matrix * displacement;

            // Add the rotated displacement to the current position
            location.position += rotated_displacement;
            info!("Position: x:{}y:{} rot: {}",location.position.x,location.position.y,location.rotation.to_degrees());
            if location.position.x >=1.0{
                movement_bus.send(global_lib::types::RoverMovement::Stop);
            }
            last_scan = scan;
        }
    });
}

