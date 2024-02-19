use std::{fs, env, default};

use config::Config;
use lazy_static::lazy_static;
use log::{info, trace};
use serde::Deserialize;


#[derive(Debug,Deserialize)]
pub struct ServerConfig{
    pub port: u16
}



impl Default for ServerConfig{
    fn default()->Self{
        Self { port: 20766 }
    }
}


#[derive(Debug,Deserialize)]
pub struct GpioConfig{
    pub motor_pins: [u8;4],
    pub en_a: u8,
    pub en_b: u8
}



#[derive(Deserialize,Debug)]
pub struct RoverConfig{
    #[serde(default)]
    pub server: ServerConfig,
    pub gpio: GpioConfig
}

impl RoverConfig{
    pub fn log_settings(&self){
        info!("{:#?}",self);
    }
}


lazy_static!{

    pub static ref CONFIG: RoverConfig = {
        let curdir = env::current_dir().unwrap();
        const FILE_NAME: &str = "roverconfig.toml";
        
        info!("Loading config file '{}/{FILE_NAME}'",curdir.as_path().to_str().unwrap() );
        
        Config::builder().add_source(config::File::with_name(FILE_NAME)).build().unwrap().try_deserialize().unwrap()
    };

}


