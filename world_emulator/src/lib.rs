use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use global_lib::{
    bus::{BusIMURotation, BusLidarScan, BusRoverMovement},
    types::{Scan, ScanMeasurement},
};
use nalgebra as na;
use tokio::{select, sync::RwLock};
mod world;

const LIDAR_CENTER_OFFSET: f32 = 0.03; //3cm

#[derive(Clone)]
struct BotLoc {
    pub position: na::Point2<f32>,
    pub rotation: f32,
}

impl BotLoc {

    pub fn forward(&mut self, distance: f32) {
        let rotation_matrix = na::Rotation2::new(self.rotation);
        let forward_local = na::Vector2::new(distance, 0.0);
        let forward_world = rotation_matrix * forward_local;
        self.position += forward_world;
    }

    pub fn get_lidar_position(&self) -> na::Point2<f32> {
        // Calculate the LIDAR position relative to the bot's center
        let lidar_offset_x = LIDAR_CENTER_OFFSET * self.rotation.cos();
        let lidar_offset_y = LIDAR_CENTER_OFFSET * self.rotation.sin();

        // Calculate the absolute LIDAR position by adding the offset to the bot's position
        let lidar_position_x = self.position.x + lidar_offset_x;
        let lidar_position_y = self.position.y + lidar_offset_y;

        // Create and return the LIDAR position as a Point2
        na::Point2::new(lidar_position_x, lidar_position_y)
    }



    pub fn rotate(&mut self, rot: f32) {
        self.rotation += rot;
        self.normalize_angle();
    }

    fn normalize_angle(&mut self) {
        const PI2: f32 = 2.0 * std::f32::consts::PI;
        self.rotation %= PI2;
        if self.rotation < 0.0 {
            self.rotation += PI2;
        }
        debug_assert!(self.rotation >= 0.0 && self.rotation <= PI2);
    }
}

pub fn emulate_world(lidar: BusLidarScan, imu: BusIMURotation, movement: BusRoverMovement) {
    let world: world::World = world::World::world1();
    let spawn = world.spawn();
    const START_ROT: f32 = 0.0f32;
    let position = Arc::new(RwLock::new(BotLoc {
        position: spawn,
        rotation: START_ROT.to_radians(),
    }));
    {
        {
            let imu2 = imu.clone();
            let pos2 = position.clone();
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_millis(300)).await;
                    let pos = pos2.read().await;
                    _ = imu2.send((
                        pos.rotation.to_degrees(),
                        SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_millis(),
                    ));
                    drop(pos)
                }
            });
        }

        let position = position.clone();
        tokio::spawn(async move {
            let delta_t = Duration::from_millis(50).as_secs_f32();

            let mut interval = tokio::time::interval(Duration::from_secs_f32(delta_t));
            let mut movement_bus = movement.subscribe();
            let mut forward = false;
            let mut rotating = 0.0f32;

            let mut rot_per_second = std::f32::consts::PI * 0.75;
            let mut meter_per_second = 0.2f32;
            loop {
                select! {
                    _=interval.tick()=>{
                        let mut position = position.write().await;
                        if forward{
                            position.forward(meter_per_second * delta_t);
                        }else if rotating != 0.0{
                            assert!(rotating.abs() == 1.0);
                            position.rotate(rot_per_second * delta_t * rotating);
                            _=imu.send((position.rotation.to_degrees(),SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()));
                        }
                    }
                    movement_result=movement_bus.recv()=>{
                        let movement = movement_result.unwrap();
                        match movement{
                            global_lib::types::RoverMovement::Forwards => {
                                forward = true;
                                rotating = 0.0
                            },
                            global_lib::types::RoverMovement::Stop => {
                                forward = false;
                                rotating = 0.0;
                            },
                            global_lib::types::RoverMovement::Right => {
                                rotating = 1.0;
                                forward = false;
                            },
                            global_lib::types::RoverMovement::Left => {
                                rotating = -1.0;
                                forward = false;
                            },
                            global_lib::types::RoverMovement::Speed(s) =>{
                                rot_per_second = std::f32::consts::PI * s;
                                meter_per_second = s / 3.0;
                            },
                            global_lib::types::RoverMovement::Teleport(point) =>{
                                 let mut position = position.write().await;
                                 position.position = point + spawn.coords;
                                 drop(position);
                            }
                        }
                    }
                }
            }
        });
    }

    {
        let position = position.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(100));
            loop {
                let mut target: Box<[ScanMeasurement]> = Box::new([Default::default(); 970]);
                let pos = position.read().await;

                world.do_scan(target.as_mut(), pos.get_lidar_position(), -pos.rotation);
                debug_assert_ne!(target[0], Default::default());
                let scan = Scan::new_now(target);
                _ = lidar.send(Arc::new(scan));
                drop(pos);

                interval.tick().await;
            }
        });
    }
}
