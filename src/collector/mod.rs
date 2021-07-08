use prometheus::{Opts, Gauge, labels};
use crate::utils::Env;

mod error;

/// Push Prometheus Pm25
///
/// # Description
/// Push the SD101 pm2.5 value to prometheus by using the push metrics feature
///
/// # Arguments
/// * `pm25` - f32
/// * `env` - &Env
pub fn push_prometheus_pm25(pm25: f32, env: &Env) -> Result<(), error::CollectorError> {
    let gauge_opts = Opts::new("pm2.5", "pm2.5 gauge")
        .const_label("value", "1");
    let gauge = Gauge::with_opts(gauge_opts)?;

    gauge.set(pm25 as f64);
    prometheus::push_metrics(
        "push pm2.5",
        labels! {"instance".to_owned() => "qi".to_owned(), },
        &env.host,
        prometheus::gather(),
        Some(prometheus::BasicAuthentication {
            username: env.username.to_owned(),
            password: env.password.to_owned()
        }))?;

    Ok(())
}

pub fn push_prometheus_pm10(pm10: f32) -> Result<(), error::CollectorError> {
    let gauge_opts = Opts::new("pm10", "pm10 gauge")
        .const_label("value", "1");

    let gauge = Gauge::with_opts(gauge_opts)?;
    gauge.set(pm10 as f64);

    Ok(())
}