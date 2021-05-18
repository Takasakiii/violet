use chrono::Utc;
use mysql::{params, prelude::Queryable};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ReportsTable<'a> {
    pub id: u64,
    pub severity: u8,
    pub title: &'a str,
    pub message: &'a str,
    pub stacktrace: Option<&'a str>,
    pub timestamp: i64,
    pub app_id: u64
}



impl<'a> ReportsTable<'a> {
    pub fn insert(severity: u8, title: &'a str, message: &'a str, stacktrace: &'a Option<String>, app_id: u64) -> Result<Self, crate::GenericError> {
        let mut conn = super::get_connection()?;
        let timestamp = Utc::now()
            .timestamp();
        conn
            .exec_drop(r"
                insert into Reports(severity_report, title_report, message_report, stacktrace_report, timestamp_report, app_report) values (:severity, :title, :message, :stacktrace, :timestamp, :app_id)
            ", params! {
                severity,
                title,
                message,
                stacktrace,
                timestamp,
                app_id
            })?;

        let id = conn
            .last_insert_id();

        let stacktrace = stacktrace
            .as_ref()
            .map(|e| &e[..]);

        let report = Self {
            id,
            severity,
            title,
            message,
            stacktrace,
            timestamp,
            app_id
        };

        Ok(report)
    }
}
