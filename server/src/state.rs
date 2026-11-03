use serde::{Serialize, Deserialize};

messaging::message_traits!{
    struct SetTemp(u32);
    struct ReportState {
	temp_mk: u32,
	power_draw_mw: u32,
    }
}

messaging::message!{TubeFurnaceProtocol {SetTemp, ReportState}}

// everything the user can see, as well as anything we're remembering on behalf of the device
struct TubeFurnace {
    operated_by: u64,
    represents: u64,
    deviceid: u64,
    power_draw_mw: u32,
    desired_temp_mk: u32,
    current_temp_mk: u32,
}
