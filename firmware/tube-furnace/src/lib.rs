//! Pico 2 implementation of [`TubeFurnace`]. Placeholder for now.

#![no_std]

use core::convert::Infallible;

use tube_furnace::TubeFurnace;

pub struct PicoFurnace {
    // TODO: thermocouple driver and heater output.
}

impl TubeFurnace for PicoFurnace {
    // TODO: use the thermocouple driver's error type.
    type Error = Infallible;

    fn temperature_c(&mut self) -> Result<f32, Infallible> {
        todo!()
    }

    fn set_heater_power(&mut self, _power: f32) {
        todo!()
    }
}
