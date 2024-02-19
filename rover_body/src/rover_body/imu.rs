use std::{
    cell::Cell,
    error::Error,
    future::Future,
    io::{self, Write},
    sync::Arc,
    time::{Duration, SystemTime},
};

use global_lib::bus::{BusIMURotation, BusLidarScan};
use i2cdev::linux::LinuxI2CError;
use linux_embedded_hal::{Delay, I2cdev};
use log::{info, warn};
use mpu6050::*;
use na::{ComplexField, Const, Vector3};
use nalgebra as na;
use tokio::{sync::RwLock, time::Instant};

#[derive(Debug)]
struct CalibratedData {
    avg: f32,
    min: f32,
    max: f32,
}

impl CalibratedData {
    //Collected data while gyro not moving
    fn new(data: &[f32]) -> Self {
        const BORDER_MULT: f32 = 1.2;

        let mut min = data[0];
        let mut max = data[0];
        for data in data.iter().cloned().skip(1) {
            if data > max {
                max = data;
            } else if data < min {
                min = data;
            }
        }

        let avg = data.iter().sum::<f32>() / data.len() as f32;
        min -= avg;
        max -= avg;
        min *= BORDER_MULT;
        max *= BORDER_MULT;

        Self { avg, min, max }
    }

    fn transform(&self, mut data: f32) -> f32 {
        data -= self.avg;
        if data > self.min && data < self.max {
            0.0
        } else {
            data
        }
    }
}

fn run_interval(interval: Duration, mut func: impl FnMut(u32) -> bool) {
    // let interval = Duration::from_millis(10);
    let mut next_time = Instant::now() + interval;
    let mut tilt_x = 0.0;
    let mut run = 0;
    loop {
        if !func(run) {
            return;
        }
        std::thread::sleep(next_time - Instant::now());
        next_time += interval;
        run += 1;
    }
}

async fn run_interval_async<Fut>(interval: Duration, func: impl Fn(u32) -> Fut)
where
    Fut: Future<Output = bool>,
{
    // let interval = Duration::from_millis(10);
    let mut interval = tokio::time::interval(interval);
    let mut run = 0;
    loop {
        if !func(run).await {
            return;
        }
        interval.tick().await;
        run += 1;
    }
}

pub struct ImuController {
    mpu: Mpu6050<I2cdev>,
    location: Arc<RwLock<Location>>
}

#[derive(Debug,Default,Clone)]
pub struct Location{
    pub rotation: f32,
    pub position: na::Point2<f32>
}

impl ImuController {
    pub fn get_location(&self) -> Arc<RwLock<Location>>{
        self.location.clone()
    }


    pub fn new() -> Self {
        info!("Setting up MPU");
        let i2c = I2cdev::new("/dev/i2c-1")
            .map_err(Mpu6050Error::I2c)
            .unwrap();

        let mut delay = Delay;
        let mut mpu = Mpu6050::new(i2c);

        mpu.init(&mut delay).unwrap();
        info!("MPU Inited");

        Self {
            mpu,
            location: Default::default()
        }
    }

    pub async fn publish_data_loop(&mut self, imu_bus: BusIMURotation) {
        let delta_t = 0.05;
        let delta_t_duration = Duration::from_secs_f32(delta_t);
        let mut delta_t_interval = tokio::time::interval(delta_t_duration);

        info!("Calibrating...");
        let calibrated_rotation = {
            let mut rotation_data = [0.0; 100];
            for rot_data in &mut rotation_data {
                let gyro = self.mpu.get_gyro().unwrap();
                *rot_data = gyro.z;
                delta_t_interval.tick().await;
            }
            CalibratedData::new(&rotation_data)
        };
        info!("Done ({calibrated_rotation:?})");

        loop {
            match self.mpu.get_gyro() {
                Ok(gyro) => {
                    let local_rotation =
                        (calibrated_rotation.transform(gyro.z) * delta_t).to_degrees();

                    let mut location_rw = self.location.write().await;
                    let rotbefore = location_rw.rotation;
                    location_rw.rotation += local_rotation + 360.0;
                    location_rw.rotation %= 360.0;
                    let location = (*location_rw).clone();
                    drop(location_rw);

                    if rotbefore != location.rotation{
                        imu_bus.send((location.rotation,SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()));
                    }
                }
                Err(e) => {
                    warn!("Gyro Error: {e:?}");
                }
            }

            delta_t_interval.tick().await;
        }
    }
}
