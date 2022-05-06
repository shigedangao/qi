use color_eyre::eyre::Result;

#[macro_use]
extern crate log;

mod sensor;
mod collector;
mod utils;
mod error;

#[derive(Debug)]
pub struct State {
    pub lap: i32,
    pub gauges: collector::Gauges
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let env = utils::load_env();
    let gauges = collector::bootstrap(&env).expect("Expect to get prometheus gauges");

    match sensor::run_sensor(State {
        lap: 0,
        gauges
    }) {
        Ok(()) => info!("Sensor has stopped but has not crashed"),
        Err(err) => error!("Error while fetching datas {}", err)
    }

    Ok(())
}
