use std::{fmt::Debug, sync::Arc, time::SystemTime};

use lazy_static::lazy_static;
use paste::paste;
use tokio::sync::broadcast::{channel, Receiver, Sender};

use crate::types;

macro_rules! generate_bus {
    (
        $($type:ty ),*
    ) => {
        paste!{

        lazy_static!(
            $(
            static ref [<BROADCAST_$type:upper>]: Sender<$type> = channel(50).0;
            )*
        );
        pub struct BusManager;
        impl BusManager{
            $(
                pub fn [<get_ $type:lower _bus>]() -> Sender<$type>{
                    let glob_broadcast = &[<BROADCAST_$type:upper>];
                    (*glob_broadcast).clone()
                }
            )*
        }
        $(
            pub type [<Bus $type>] = Sender<$type>;
        )*
    }
    };
}

type IMURotation = (f32,u128);
type LidarScan = Arc<super::types::Scan>;
type RoverMovement = types::RoverMovement;

generate_bus!(LidarScan, IMURotation,RoverMovement);

mod test {
    use std::time::Duration;

    use super::*;
    #[tokio::main]
    #[test]
    async fn test() {
        let mut bus = BusManager::get_imurotation_bus();
        let mut buscpy = bus.clone();
        tokio::spawn(async move {
            let mut sub = buscpy.subscribe();
            loop{
                let msg = sub.recv().await.unwrap();
                println!("first got {msg:?}");
                tokio::task::yield_now().await;
            }
        });

        tokio::spawn(async move {
            let bus = BusManager::get_imurotation_bus();
            let mut sub = bus.subscribe();
            loop{
                let msg = sub.recv().await.unwrap();
                println!("second got {msg:?}");
                tokio::time::sleep(Duration::from_millis(1000)).await;

            }
        });
        // may be a race condition!. Needs to wait so that eventloop processes the on_message calls
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        for i in 0..10 {
            println!("Broadcasting {i}");
            bus.send((i as f32,0));
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        // this call gives off again so that eventloop resolves on_message
        tokio::time::sleep(Duration::from_millis(10000)).await;
    }

    #[tokio::main]
    #[test]
    async fn test_bounded() {
        let mut bus = BusManager::get_imurotation_bus();
        let mut buscpy = bus.clone();
        let mut buscpy3 = bus.clone();

        tokio::spawn(async move {
            let mut sub = BusManager::get_imurotation_bus().subscribe();
            loop {
                let msg = sub
                    .recv()
                    .await
                    .unwrap();
                println!("second got {msg:?}")
            }
        });
        // may be a race condition!. Needs to wait so that eventloop processes the on_message calls
        tokio::time::sleep(Duration::from_millis(1000)).await;

        for i in 0..10 {
            println!("Broadcasting {i}");
            bus.send((i as f32,0));
        }

        // this call gives off again so that eventloop resolves on_message
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
}
