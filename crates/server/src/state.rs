use std::num::NonZeroU64;

// everything the user can see, as well as anything we're remembering on behalf of the device
pub struct TubeFurnace {
    operated_by: Option<NonZeroU64>,
    controller: tokio::sync::mpsc::Sender<()>, // treat as mutable access
    desired_temp_k: f64,
    current_temp_k: f64,
    desired_temp_dirty: bool,
}

impl TubeFurnace {
    fn new(desired_temp_k: f64, current_temp_k: f64, controller: tokio::sync::mpsc::Sender<()>) -> Self {
	TubeFurnace {
	    operated_by: None,
	    controller,
	    desired_temp_k,
	    current_temp_k,
	    desired_temp_dirty: false,
	}
    }

    fn controller(&self, operator: NonZeroU64) -> Option<&tokio::sync::mpsc::Sender<()>> {
	if Some(operator) == self.operated_by {
	    Some(&self.controller)
	} else {
	    None
	}
    }
}

pub enum Tool {
    TubeFurnace(TubeFurnace),
}

impl Tool {
    pub fn name(&self) -> &str {
	match self {
	    Self::TubeFurnace(_) => "Tube furnace",
	}
    }
}
