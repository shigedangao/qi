use std::sync::{Arc, Mutex};

#[macro_use]
extern crate log;

mod sensor;
mod collector;
mod utils;

fn main() {
    env_logger::init();
    let gauges = match collector::bootstrap() {
        Ok(res) => res,
        Err(err) => panic!("{}", err)
    };

    let lap = Arc::new(Mutex::new(0));
    match sensor::run_sensor(lap, &gauges) {
        Ok(()) => println!("Ok"),
        Err(err) => println!("{:?}", err)
    }
}
