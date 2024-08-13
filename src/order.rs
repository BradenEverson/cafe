use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Order {
    order_time: DateTime<Local>,
    special: Option<SpecialAttributes>,
    size: Size,
}

// TODO(Once we know what GPIO pin controls these buttons, asign that value to the enum)
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecialAttributes {
    ColdBrew,
    Strong,
}

// TODO(Once we know what GPIO pin controls these buttons, asign that value to the enum)
#[repr(usize)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Size {
    EightOunce,
    TenOunce,
    #[default]
    TwelveOunce,
}
