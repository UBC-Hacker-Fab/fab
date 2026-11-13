use crate::macros::{message, message_derives};

message_derives! {
    pub enum TestPing {
        Ping(u8),
        Pong(u8),
    }

    pub struct TestStruct {

    }

    pub enum PinState {
        High,
        Low,
    }
}

message! {
    pub ToolTestMessage {
        TestPing,
        TestStruct,
        PinState,
    }
}
