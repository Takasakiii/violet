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

impl Into<u8> for Severity {
    fn into(self) -> u8 {
        match self {
            Self::NoDefined => 0,
            Self::Severe => 1,
            Self::Error => 2,
            Self::Warning => 3,
            Self::Info => 4,
            Self::Verbose => 5
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EventTrackerReceive {
    pub app_id: Option<u64>,
    pub severity: Severity,
    pub title: String,
    pub message: String,
    pub stacktrace: Option<String>,
}

#[derive(Serialize)]
pub struct ErrPayload {
    pub message: String
}
