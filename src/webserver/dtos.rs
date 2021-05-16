use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Severity {
    NoDefined = 0,
    Severe = 1,
    Error = 2,
    Warning = 3,
    Info = 4,
    Verbose = 5
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EventTrackerReceive {
    pub app_id: Option<u64>,
    pub severity: Severity,
    pub title: String,
    pub message: String,
    pub stacktrace: Option<String>,
}
