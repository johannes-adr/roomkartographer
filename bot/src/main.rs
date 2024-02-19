use global_lib::bus::{BusLidarScan, BusManager, BusRoverMovement, BusIMURotation};
use global_lib::CONFIG;
use log::info;
mod position_tracker;
#[tokio::main]
async fn main() {
    _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting...");
    CONFIG.log_settings();
    let lidarbus = BusManager::get_lidarscan_bus();
    let imubus = BusManager::get_imurotation_bus();
    let movement_bus = BusManager::get_rovermovement_bus();
    world_emulator::emulate_world(lidarbus.clone(),imubus.clone(),movement_bus.clone());
    // spawn_real(lidarbus.clone(),movement_bus.clone(),imubus.clone());

    let lidarbus2 = lidarbus.clone();
    tokio::spawn(async move{ webserver::start_server(lidarbus2,movement_bus,imubus).await });
    let mut lidarlistener = lidarbus.subscribe();
    loop {
        let data = lidarlistener.recv().await.unwrap();
        // println!("{}", data.get_points().len())
    }

    // let bus = bus::Bus::new();
}

// fn spawn_real(bus_lidar: BusLidarScan,movement_bus: BusRoverMovement, imu_bus: BusIMURotation) {
//     tokio::task::spawn_blocking(|| {
//         let mut lidar = lidar::rplidarwrapper::RpLidar::new();
//         rover_body::start_rover_body_and_imu(movement_bus, imu_bus,bus_lidar.clone());
//         lidar.broadcast_blocking(bus_lidar);
//     });
// }