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

    // Tube furnace
    pub struct SetTemp {
	pub temp_k: f64,
    }
    
    pub struct SetRamp {
	pub rate_k_per_s: f64,
    }

    // heartbeat required to keep connection alive
    pub struct TubeFurnaceReportStatus {
	pub temp_k: f64,
    }
    
    pub struct TubeFurnaceReportIntent {
	pub temp_k: f64,
	pub rate_k_per_s: f64,
    }

    pub enum DeviceType {
	TubeFurnace,
    }
    pub struct DeviceApikey {
	device_type: DeviceType,
	api_key: [u8; 8],
    }
}

message! {
    pub ToolTestMessage {
        TestPing,
        TestStruct,
        PinState,
    }
}

message! {
    pub ServerToTubeFurnace {
	SetTemp,
	SetRamp,
    }
}

message! {
    pub TubeFurnaceToServer {
	TubeFurnaceReportStatus,
    }
}

message! {
    pub DeviceSetup {
	DeviceApikey,
    }
}

