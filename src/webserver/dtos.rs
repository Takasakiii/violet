use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serenity::utils::Color;

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

impl From<Severity> for Color {
    fn from(s: Severity) -> Self {
        match s {
            Severity::NoDefined => Color::LIGHTER_GREY,
            Severity::Severe => Color::from_rgb(0, 0, 0),
            Severity::Error => Color::RED,
            Severity::Warning => Color::GOLD,
            Severity::Info => Color::BLUE,
            Severity::Verbose => Color::from_rgb(0, 255, 0)
        }
    }
}


impl From<Severity> for u8 {
    fn from(val: Severity) -> Self {
        match val {
            Severity::NoDefined => 0,
            Severity::Severe => 1,
            Severity::Error => 2,
            Severity::Warning => 3,
            Severity::Info => 4,
            Severity::Verbose => 5
        }
    }
}

impl From<Severity> for String {
    fn from(val: Severity) -> Self {
        match val {
            Severity::NoDefined => "Não definido".into(),
            Severity::Severe => "Erro Severo".into(),
            Severity::Error => "Erro".into(),
            Severity::Warning => "Aviso".into(),
            Severity::Info => "Informação".into(),
            Severity::Verbose => "Dado de depuração".into()
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EventTrackerReceive {
    pub app_id: Option<u64>,
    pub severity: Severity,
    pub title: String,
    pub message: String,
    pub stacktrace: Option<String>
}

#[derive(Serialize)]
pub struct ErrPayload {
    pub message: String
}
