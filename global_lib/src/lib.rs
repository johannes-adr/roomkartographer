use serde::{Serialize, Deserialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
mod rover_config;
pub use rover_config::*;
use tokio::task::JoinHandle;



pub mod types;
pub mod bus;


// #[derive(Serialize,Deserialize,Debug)]
// pub enum Packet{
//     Forwards,
//     Left,
//     Right,
//     Backwards,
//     Stop,
//     Speed(f64),
//     RotateTo(f32)
// }


// impl Packet{
//     pub async fn read_from<T: AsyncReadExt + std::marker::Unpin>(stream: &mut T) -> Result<Self>{
//         let len = stream.read_u32().await? as usize;
//         let mut data = vec![0u8;len];
//         let received_bytes = stream.read_exact(&mut data).await?;
//         if received_bytes != len{
//             return Err(format!("Expected {len} bytes, received {received_bytes}").into())
//         }

//         Ok(bincode::deserialize(&data)?)
//     }

//     pub async fn write_to<T: AsyncWriteExt + std::marker::Unpin>(&self,stream: &mut T) -> Result<()>{
//         let mut data = bincode::serialize(&self)?;
//         stream.write_u32(data.len() as u32).await?;
//         stream.write_all(&mut data).await?;
//         Ok(())
//     }
// }










pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T,Error>;


use std::fmt::Debug;

pub use log::{debug,warn,info,trace,error};


use tokio::sync::{oneshot, Mutex};
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;




pub struct ReturnedOneshot<T> {
    sender: oneshot::Sender<()>,
    executed_future: JoinHandle<T>,
}

impl<T: 'static + Send> ReturnedOneshot<T> {
    pub fn new<F,Fut>(future_fn: F) -> Self
    where
        F: FnOnce(oneshot::Receiver<()>) -> Pin<Box<Fut>>,
        Fut: Future<Output = T> + Send + 'static
    {
        let (sender, receiver) = oneshot::channel();
        let receiver_future = future_fn(receiver);
        let executed_future = tokio::spawn(receiver_future);
        ReturnedOneshot {
            sender,
            executed_future,
        }
    }

    pub async fn call(self) -> T {
        self.sender.send(()).unwrap(); // Handle this unwrap appropriately in production
        let t = self.executed_future.await.unwrap();
        t
    }
}
