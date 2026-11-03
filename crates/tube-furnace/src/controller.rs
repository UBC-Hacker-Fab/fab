//! Heater control.

use crate::TubeFurnace;

/// Proportional temperature controller.
pub struct Controller {
    /// Target temperature in °C.
    pub setpoint_c: f32,
    /// The heater is forced off at or above this temperature, in °C.
    pub limit_c: f32,
    /// Heater power per °C below the setpoint.
    pub gain: f32,
}

impl Controller {
    /// Reads the temperature and sets the heater power.
    pub fn step<F: TubeFurnace>(&self, furnace: &mut F) {
        let power = match furnace.temperature_c() {
            Ok(temp) if temp < self.limit_c => {
                ((self.setpoint_c - temp) * self.gain).clamp(0.0, 1.0)
            }
            // Over the limit or NaN.
            Ok(_) => 0.0,
            Err(_) => 0.0,
        };
        furnace.set_heater_power(power);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimFurnace;

    fn controller() -> Controller {
        Controller {
            setpoint_c: 1000.0,
            limit_c: 1100.0,
            gain: 0.1,
        }
    }

    #[test]
    fn reaches_setpoint() {
        let ctrl = controller();
        let mut sim = SimFurnace::new();
        for _ in 0..(4 * 60 * 60) {
            ctrl.step(&mut sim);
            sim.advance(1.0);
        }
        assert!((sim.temp_c - 1000.0).abs() < 10.0, "got {} °C", sim.temp_c);
    }

    #[test]
    fn heater_off_when_sensor_fails() {
        let ctrl = controller();
        let mut sim = SimFurnace::new();
        sim.sensor_broken = true;
        ctrl.step(&mut sim);
        assert_eq!(sim.heater_power(), 0.0);
    }

    #[test]
    fn heater_off_when_reading_is_nan() {
        let ctrl = controller();
        let mut sim = SimFurnace::new();
        sim.temp_c = f32::NAN;
        ctrl.step(&mut sim);
        assert_eq!(sim.heater_power(), 0.0);
    }
}
