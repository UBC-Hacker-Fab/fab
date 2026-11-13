#![cfg_attr(not(test), no_std)]

mod controller;
mod sim;

pub use controller::Controller;
pub use sim::{SensorBroken, SimFurnace};

pub trait TubeFurnace {
    /// Error from reading the temperature.
    type Error;

    /// Reads the temperature in °C.
    fn temperature_c(&mut self) -> Result<f32, Self::Error>;

    /// Sets the heater power from 0.0 (off) to 1.0 (full).
    fn set_heater_power(&mut self, power: f32);
}
