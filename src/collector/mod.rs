use prometheus_exporter::prometheus::register_gauge;
use prometheus::Gauge;
use crate::utils;

// Easier to manipulate with the gauges...
pub type Gauges = (Gauge, Gauge);

/// Bootstrap the prometheus exporter
///     Create a prometheus exporter instance which will listen to metrics created
pub fn bootstrap() -> Result<Gauges, Box<dyn std::error::Error>> {
    info!("Initializing prometheus exporter");

    let env = utils::load_env()?;
    prometheus_exporter::start(env.host.parse().unwrap())?;
    create_gauge()
}

/// Create Gauge 
///     Create the gauge which will be updated later
fn create_gauge() -> Result<Gauges, Box<dyn std::error::Error>> {
    info!("Create prometheus probe");

    let pm25 = register_gauge!("particle_pm25", "set particle pm25")?;
    let pm10 = register_gauge!("particle_pm10", "set particle pm10")?;

    Ok((pm25, pm10))
}