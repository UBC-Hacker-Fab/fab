//! Packing of messages sent by the comms board.

#![cfg_attr(not(test), no_std)]

/// A message sent by the comms board.
pub struct Message {
    pub counter: u8,
}

impl Message {
    /// Packs the message into bytes: a `0xA5` header, then the counter.
    pub fn pack(&self) -> [u8; 2] {
        [0xA5, self.counter]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packs_header_then_counter() {
        assert_eq!(Message { counter: 7 }.pack(), [0xA5, 7]);
    }
}
