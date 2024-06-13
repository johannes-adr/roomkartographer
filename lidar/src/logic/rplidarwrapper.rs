use global_lib::bus;
use log::{info, trace, warn};
use nalgebra as na;
use rplidar_drv::RplidarDevice;
use serialport::SerialPort;

use tokio::sync::RwLock;

use global_lib::types::Scan;

use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

type TargetBuffer = Arc<RwLock<Arc<Scan>>>;
const SERIAL_PORT1: &str = "/dev/ttyUSB0"; //LINUX
// const SERIAL_PORT1: &str ="/dev/tty.SLAB_USBtoUART"; //MAC
pub struct RpLidar {
    lidar: RplidarDevice<dyn SerialPort>,
}

//AsyncRpLidar will send one None through scan channel to broadcast its ready state!
impl RpLidar {
    pub fn new() -> Self {
        info!("Starting Lidar...");
        let mut serial_port = get_port();
        serial_port.write_data_terminal_ready(false).unwrap();
        let mut lidar: RplidarDevice<dyn SerialPort> = RplidarDevice::with_stream(serial_port);

        let actual_mode = lidar
            .start_scan()
            .expect("failed to start scan in standard mode");

        info!("Started lidar scan in mode `{}`", actual_mode.name);

        Self { lidar }
    }

    pub fn broadcast_blocking(&mut self,bus: bus::BusLidarScan) {
        loop {
            match self.lidar.grab_scan() {
                Ok(scan) => {
                    if scan.len() <= 1{
                        continue;
                    }
                    let scan: Scan = scan.into();
                    bus.send(Arc::new(scan));
                }
                Err(e) => warn!("Error grabbing scan - '{e}' - retrying"),
            }
        }
    }
}

fn get_port() -> Box<dyn SerialPort> {
    // info!(
    //     "Avalible ports: {:?}",
    //     serialport::available_ports().unwrap()
    // );
    // std::process::exit(0);
    trace!("Getting port {SERIAL_PORT1}");
    let port = serialport::new(SERIAL_PORT1, 115_200)
        .timeout(Duration::from_millis(10))
        .open()
        .unwrap();

    trace!("Done getting port");
    port
}
