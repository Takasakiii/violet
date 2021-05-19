use chrono::Utc;
use mysql::{params, prelude::Queryable};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ReportsTable {
    pub id: u64,
    pub severity: u8,
    pub title: String,
    pub message: String,
    pub stacktrace: Option<String>,
    pub timestamp: i64,
    pub app_id: u64
}



impl ReportsTable {
    pub fn insert(severity: u8, title: &str, message: &str, stacktrace: &Option<String>, app_id: u64) -> Result<Self, crate::GenericError> {
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

        let report = Self {
            id,
            severity,
            title: title.into(),
            message: message.into(),
            stacktrace: stacktrace
                .as_ref()
                .map(|e| e.to_string()),
            timestamp,
            app_id
        };

        Ok(report)
    }

    pub fn get_last_25(id_app: u64, author_id: u64) -> Result<Vec<Self>, crate::GenericError> {
        let mut conn = super::get_connection()?;
        let result = conn
            .exec_map(r"
                select r.* from Reports r join Apps a on a.id_app = r.app_report where r.app_report = :app and a.id_user = :owner order by timestamp_report desc limit 25
            ", params! {
                "app" => id_app,
                "owner" => author_id
            },
            |(id, severity, title, message, stacktrace, timestamp, app)| {
                Self {
                    id,
                    severity,
                    title,
                    message,
                    stacktrace,
                    timestamp,
                    app_id: app
                }
            })?;

        Ok(result)
    }

    pub fn get(id_event: u64, author_id: u64) -> Option<Self> {
        let mut conn = super::get_connection()
            .ok()?;
        let result = conn
            .exec_map(r"
                select r.* from Reports r join Apps a on a.id_app = r.app_report where r.id_report = :report and a.id_user = :owner
            ", params! {
                "report" => id_event,
                "owner" => author_id
            }, |(id, severity, title, message, stacktrace, timestamp, app)| {
                Self {
                    id,
                    severity,
                    title,
                    message,
                    stacktrace,
                    timestamp,
                    app_id: app
                }
            })
                .ok()?
                .pop();
        result
    }
}
