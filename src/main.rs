use color_eyre::eyre::Result;

#[macro_use]
extern crate log;

mod sensor;
mod collector;
mod utils;

#[derive(Debug)]
pub struct State {
    pub lap: i32
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();
    
    let gauges = match collector::bootstrap() {
        Ok(res) => res,
        Err(err) => panic!("{}", err)
    };

    match sensor::run_sensor(State { lap: 0 }, Some(gauges)) {
        Ok(()) => info!("Sensor has stop but has not crashed"),
        Err(err) => error!("Error while fetching datas {}", err)
    }

    Ok(())
}
