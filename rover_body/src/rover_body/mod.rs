use std::{
    cell::Cell,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::Arc,
    time::Duration,
};

use global_lib::bus::BusRoverMovement;
use global_lib::types::RoverMovement;
use global_lib::{ReturnedOneshot, CONFIG};
use log::{debug, error, info, log_enabled, trace, Level};
use rppal::gpio::{Gpio, OutputPin};
use tokio::{
    join,
    sync::Mutex,
    task,
    time::{interval, Interval},
};

pub mod imu;

const FULLSPEEDDURATION: f32 = 1.0;
const MAX_SPEED: f32 = 0.45;
const START_SPEED: f32 = 0.3;
const UPDATE_SECS: f32 = 0.025;

const NUM_OF_STEPS: f32 = FULLSPEEDDURATION / UPDATE_SECS;
const STEP_SIZE: f32 = MAX_SPEED / NUM_OF_STEPS;

pub struct ChachedOutputPin {
    pin: OutputPin,
    high: bool,
}

/*

*/

impl From<OutputPin> for ChachedOutputPin {
    fn from(value: OutputPin) -> Self {
        Self {
            high: value.is_set_high(),
            pin: value,
        }
    }
}

impl ChachedOutputPin {
    ///Returns true if something changed
    pub fn set_state(&mut self, high: bool) -> bool {
        if high {
            // self.set_high()
            self.pin.set_high();
        } else {
            self.pin.set_low();
        }

        return true;
    }
    ///Set high if state is low. Returns true if something changed
    pub fn set_high(&mut self) -> bool {
        if self.high {
            return false;
        }
        self.high = true;
        self.pin.set_high();
        return true;
    }

    ///Set low if state is high. Returns true if something changed
    pub fn set_low(&mut self) -> bool {
        if !self.high {
            return false;
        }
        self.high = false;
        self.pin.set_low();
        return true;
    }
}

impl Deref for ChachedOutputPin {
    type Target = OutputPin;

    fn deref(&self) -> &Self::Target {
        &self.pin
    }
}

impl DerefMut for ChachedOutputPin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pin
    }
}

pub struct RoverBody {
    control_pins: [OutputPin; 4],
    en_a: OutputPin,
    en_b: OutputPin,
    movement_bus: BusRoverMovement,
    current_movement: RoverMovement,
}

impl RoverBody {
    pub fn new(gpio: &Gpio, movement_bus: BusRoverMovement) -> Self {
        let control_pins = CONFIG
            .gpio
            .motor_pins
            .map(|p| gpio.get(p).unwrap().into_output());
        Self {
            control_pins,
            movement_bus,
            current_movement: RoverMovement::Stop,
            en_a: gpio.get(CONFIG.gpio.en_a).unwrap().into_output(),
            en_b: gpio.get(CONFIG.gpio.en_b).unwrap().into_output(),
        }
    }

    pub async fn listen_bus(&mut self) {
        info!("Body listening");
        self.stop();
        let mut receiver = self.movement_bus.subscribe();
        let mut interval = interval(Duration::from_secs_f32(UPDATE_SECS));

        let mut speed = START_SPEED;
        loop {
            
            if let Ok(move_instruction) = receiver.try_recv(){
                info!("Body got instruction {move_instruction:?}");
                self.current_movement = move_instruction;
                speed = 0.0;
                self.speed(speed);
                match move_instruction {
                    global_lib::types::RoverMovement::Forwards => self.forwards(),
                    global_lib::types::RoverMovement::Stop => self.stop(),
                    global_lib::types::RoverMovement::Right => self.right(),
                    global_lib::types::RoverMovement::Left => self.left(),
                    global_lib::types::RoverMovement::Speed(s) => panic!(),
                }
            }else{
                self.speed(speed);
            };
            interval.tick().await;
            speed += STEP_SIZE;
            if speed > MAX_SPEED{
                speed = MAX_SPEED;
            }

        }
    }

    #[inline]
    fn apply_state(&mut self, state: [bool; 4]) -> bool {
        let mut changed = false;

        state
            .iter()
            .zip(self.control_pins.iter_mut())
            .for_each(|(state, pin)| {
                info!("Setting pinn {} to {state}", pin.pin());
                // pin.set_state(*state)

                if *state {
                    pin.set_high();
                } else {
                    pin.set_low();
                }
                info!("{}", pin.is_set_high());
            });

        changed
    }

    fn speed(&mut self, mut speed: f32) {
        if speed >= 1.0 {
            speed = 0.99;
        } else if speed <= 0.0 {
            speed = 0.01;
        }
        self.en_a.set_pwm_frequency(490.0, speed as f64).unwrap();
        self.en_b.set_pwm_frequency(490.0, speed as f64).unwrap();
    }

    // pub async fn rotate_to(&mut self, target_angle: f32) {
    //     const TOLERANCE: f32 = 4.0;
    //     let mut last_rotation = Option::None;
    //     let mut tries = 0;
    //     loop {
    //         let current_rotation = self.imu.get_rotation().await;
    //         if let Some(s) = last_rotation{
    //             if s == current_rotation{
    //                 if tries >= 100{
    //                     error!("Robot is Stuck / IMU not working");
    //                     self.stop();
    //                     return;
    //                 }
    //                 // trace!("Rotation tries: {tries}",);
    //                 tries += 1;

    //             }else{
    //                 tries = 0;
    //             }
    //         }
    //         last_rotation = Some(current_rotation);
    //         // Calculate the clockwise and counter-clockwise differences
    //         let diff_clockwise = if current_rotation <= target_angle {
    //             target_angle - current_rotation
    //         } else {
    //             360.0 - current_rotation + target_angle
    //         };

    //         let diff_counter_clockwise = if current_rotation >= target_angle {
    //             current_rotation - target_angle
    //         } else {
    //             360.0 - target_angle + current_rotation
    //         };

    //         // Check if within tolerance
    //         if diff_clockwise < TOLERANCE || diff_counter_clockwise < TOLERANCE {
    //             self.stop();
    //             return;
    //         }

    //         let diff =
    //         // Decide direction
    //         if diff_clockwise < diff_counter_clockwise {
    //             // It is closer to turn right (clockwise)
    //             self.right();
    //             diff_clockwise
    //         } else {
    //             // It is closer to turn left (counter-clockwise)
    //             self.left();
    //             diff_counter_clockwise
    //         };

    //         // let mut speed = diff / 60.0;
    //         // if speed > 0.7{
    //         //     speed = 0.7
    //         // }else if speed <= 0.4{
    //         //     speed = 0.4;
    //         // }

    //         println!("{}",current_rotation);

    //         tokio::time::sleep(Duration::from_millis(10)).await;
    //     }
    // }

    fn stop(&mut self) {
        if self.apply_state([false, false, false, false]) {
            trace!("Stopping");
        }
    }

    fn forwards(&mut self) {
        if self.apply_state([true, false, true, false]) {
            trace!("Moving forwardss");
        }
    }

    // pub fn controlled_forward(mut self) -> ReturnedOneshot<Self> {
    //     let one = ReturnedOneshot::new(|mut stop_receiver: oneshot::Receiver<_>|{
    //         Box::pin(async move {
    //             let mut scan_before = self.lidar.grab_scan().await;
    //             let mut interval = tokio::time::interval(Duration::from_millis(50));
    //             self.forwards();

    //             loop {
    //                 tokio::select! {
    //                     _ = interval.tick() => {
    //                         println!("Tick {} - {:?}",scan_before.get_points().len(),scan_before.get_time());
    //                         let (curr_scan, rotation) = tokio::join!(self.lidar.grab_new_scan(&scan_before), self.imu.get_rotation());
    //                         let start = Instant::now();
    //                         let val = icp(&scan_before, &curr_scan, rotation);

    //                         println!("Icp {val} took: {:?}",start.elapsed());
    //                         scan_before = curr_scan;
    //                     }
    //                     _ = &mut stop_receiver => {
    //                         println!("GotStop");
    //                         // Stop signal received
    //                         break;
    //                     }
    //                 }
    //             }
    //             self.stop();
    //             self
    //         })
    //     });
    //     one
    //     // stop_sender
    // }

    fn left(&mut self) {
        if self.apply_state([true, false, false, true]) {
            trace!("Moving forwards");
        }
    }

    fn right(&mut self) {
        if self.apply_state([false, true, true, false]) {
            trace!("Moving forwards");
        }
    }
}
