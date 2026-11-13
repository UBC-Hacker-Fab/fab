# Adding a Tool

Each tool is split into two crates so its logic can be tested without the
hardware:

- `crates/<tool>/`: a trait describing the tool, the control logic, and a
  simulation. Tests run on your computer with `cargo test`.
- `firmware/<tool>/`: implements the trait for the real board.

The tube furnace (`crates/tube-furnace/` and `firmware/tube-furnace/`) is the
reference example.

## 1. Create the crate

Copy [crates/tube-furnace/Cargo.toml](../crates/tube-furnace/Cargo.toml) to
`crates/<tool>/` and change the name. The root workspace includes everything in
`crates/` automatically.

Start `src/lib.rs` with the `no_std` line from
[crates/tube-furnace/src/lib.rs](../crates/tube-furnace/src/lib.rs). It keeps the
crate usable on the microcontroller, which has no standard library, while still
allowing it in tests.

## 2. Define the trait

The trait lists what the tool can read and do. See `TubeFurnace` in
[crates/tube-furnace/src/lib.rs](../crates/tube-furnace/src/lib.rs).

- Use physical units (°C, not raw ADC counts) and put the unit in the name,
  e.g. `temperature_c`.
- Return a `Result` for anything that can fail, with `type Error` so each
  implementation picks its own error type.

## 3. Add a simulation

Write a struct that implements the trait and fakes the physics. See
`SimFurnace` in [crates/tube-furnace/src/sim.rs](../crates/tube-furnace/src/sim.rs).

- Make an `advance(dt_s)` method that steps time forward, so tests can run
  hours of simulated time instantly.
- Make the state of the world `pub` (e.g. `temp_c`) so tests can set up
  scenarios.
- Add flags for faults you want to test (e.g. `sensor_broken`).
- Keep outputs private with a getter (e.g. `heater_power()`), so only the
  trait can change them.
- Check that inputs follow the trait's rules with `debug_assert!` (e.g. heater
  power between 0.0 and 1.0), so tests catch logic that breaks them.

## 4. Write the logic and tests

Write the logic as generic over the trait, so it works with both the simulation
and the real board. See `Controller::step` and its tests in
[crates/tube-furnace/src/controller.rs](../crates/tube-furnace/src/controller.rs).

Tests run the logic against the simulation. Cover normal operation and the
failures that matter for safety: sensor errors, NaN readings, values out of
range.

```bash
cargo test -p <tool>
```

## 5. Add the firmware crate

Add the crate to `members` and `[workspace.dependencies]` in
[firmware/Cargo.toml](../firmware/Cargo.toml), the same way as `tube-furnace`.

Copy [firmware/tube-furnace/Cargo.toml](../firmware/tube-furnace/Cargo.toml)
and change the names. The package name needs the `-firmware` suffix because
package names must be unique. `test = false` is needed because tests can't run
on the chip.

In `src/lib.rs`, implement the trait for the board. Until the hardware code is
written, the methods can be `todo!()` (see
[firmware/tube-furnace/src/lib.rs](../firmware/tube-furnace/src/lib.rs)).
`todo!()` panics if it runs, so don't flash a stub.

> TODO: Document adding the firmware binary once a tool has one. Until then,
> see [firmware/comms-board/](../firmware/comms-board/) for a working binary.

## Before opening a PR

Run the same checks as CI, listed in
[.github/workflows/linux-ci.yml](../.github/workflows/linux-ci.yml).
