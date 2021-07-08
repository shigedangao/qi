use std::sync::{Arc, Mutex};

mod sensor;

fn main() {
    let lap = Arc::new(Mutex::new(0));
    
    match sensor::run_sensor(lap) {
        Ok(()) => println!("Ok"),
        Err(err) => println!("{:?}", err)
    }
}
