//! Simulated tube furnace for testing. Simple illustrative version for now.

use crate::TubeFurnace;

const ROOM_C: f32 = 20.0;

// Placeholder values. At full power the furnace levels off at
// 20 + 0.25 / 0.0002 = 1270 °C.

/// °C per second at full power.
const FULL_POWER_HEATING: f32 = 0.25;

/// Fraction of the difference to room temperature lost per second.
const HEAT_LOSS: f32 = 0.0002;

/// Simulated furnace. Time only passes when [`SimFurnace::advance`] is called.
pub struct SimFurnace {
    /// Temperature in °C.
    pub temp_c: f32,
    /// Makes [`TubeFurnace::temperature_c`] return an error.
    pub sensor_broken: bool,
    power: f32,
}

/// Returned when `sensor_broken` is set.
#[derive(Debug, PartialEq)]
pub struct SensorBroken;

impl SimFurnace {
    /// Starts at room temperature with the heater off.
    pub fn new() -> Self {
        Self {
            temp_c: ROOM_C,
            sensor_broken: false,
            power: 0.0,
        }
    }

    /// Last power passed to [`TubeFurnace::set_heater_power`].
    pub fn heater_power(&self) -> f32 {
        self.power
    }

    /// Advances the simulation by `dt_s` seconds.
    pub fn advance(&mut self, dt_s: f32) {
        let heating = self.power * FULL_POWER_HEATING;
        let cooling = (self.temp_c - ROOM_C) * HEAT_LOSS;
        self.temp_c += (heating - cooling) * dt_s;
    }
}

impl Default for SimFurnace {
    fn default() -> Self {
        Self::new()
    }
}

impl TubeFurnace for SimFurnace {
    type Error = SensorBroken;

    fn temperature_c(&mut self) -> Result<f32, SensorBroken> {
        if self.sensor_broken {
            Err(SensorBroken)
        } else {
            Ok(self.temp_c)
        }
    }

    fn set_heater_power(&mut self, power: f32) {
        debug_assert!((0.0..=1.0).contains(&power), "power out of range: {power}");
        self.power = power;
    }
}
