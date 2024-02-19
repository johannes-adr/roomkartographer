use serde::{Deserialize, Serialize};
use std::fmt::Debug;
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WebLidarScan {
    pub points: Box<[WebPoint]>,
    pub time: String,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct IMURotationData {
    pub rotation: f32,
    pub time: String,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum WebPacket {
    LidarScan(WebLidarScan),
    IMURotation(IMURotationData),
    Movement(WebMovement),
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum WebMovement {
    Forward,
    Stop,
    Left,
    Right,
    Speed(f32),
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WebPoint {
    pub x: f32,
    pub y: f32,
}
