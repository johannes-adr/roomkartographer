pub use rplidar_drv::ScanPoint;
mod scan;
pub use scan::*;



#[derive(Clone,Copy,Debug)]
pub enum RoverMovement{
    Forwards,
    Stop,
    Right,
    Left,
    Speed(f32)
}