use std::net::SocketAddr;

use prometheus_exporter::prometheus::{
    core::{AtomicF64, GenericGauge},
    register_gauge
};
use crate::utils::Env;
use crate::error::SensorError;

// Easier to manipulate with the gauges...
pub type Gauges = (GenericGauge::<AtomicF64>, GenericGauge::<AtomicF64>);

/// Bootstrap the prometheus exporter
///     Create a prometheus exporter instance which will listen to metrics created
/// 
/// # Arguments
/// * `env` - &Env
pub fn bootstrap(env: &Env) -> Result<Gauges, SensorError> {
    info!("Initializing prometheus exporter");

    let addr = env.host.parse::<SocketAddr>()?;
    create_gauge(addr)
}

/// Create Gauge 
///     Create the gauge which will be updated later
/// 
/// # Arguments
/// * `addr` - SocketAddr
fn create_gauge(addr: SocketAddr) -> Result<Gauges, SensorError> {
    info!("Create prometheus probe");

    let pm25 = register_gauge!("particle_pm25", "set particle pm25")?;
    let pm10 = register_gauge!("particle_pm10", "set particle pm10")?;

    // Start exporter
    prometheus_exporter::start(addr)?;

    Ok((pm25, pm10))
}