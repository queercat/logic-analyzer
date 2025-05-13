use std::error::Error;
use rppal::gpio::{Gpio};
use std::thread;
use std::time::{Duration, SystemTime};
use csv::{Writer, ByteRecord};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct Row {
    bit: u8,
    time: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new()?;
    let pin = gpio.get(17)?.into_input_pulldown();
    let mut writer = Writer::from_path("signal.csv")?;
    let now = SystemTime::now();

    loop {
        let row = Row {
            bit: pin.read() as u8,
            time: now.elapsed()?.as_secs_f64()
        };
        writer.serialize(row)?;
        writer.flush()?;
    }
}
