#[macro_use]
extern crate log;

mod sensor;
mod collector;
mod utils;

#[derive(Debug)]
pub struct State {
    pub lap: i32
}

fn main() {
    env_logger::init();
    let gauges = match collector::bootstrap() {
        Ok(res) => res,
        Err(err) => panic!("{}", err)
    };

    match sensor::run_sensor(State { lap: 0 }, Some(gauges)) {
        Ok(()) => println!("Ok"),
        Err(err) => println!("{:?}", err)
    }
}
