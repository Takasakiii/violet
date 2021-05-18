mod app_table;
mod reports_table;

pub use app_table::AppTable;
pub use reports_table::ReportsTable;

use mysql::{Pool, PooledConn, prelude::Queryable};
use crate::config;

static mut CONNECTION: Option<Pool> = None;

pub fn get_connection() -> Result<PooledConn, mysql::Error> {
    unsafe {
        match &CONNECTION {
            Some(pool) => Ok(pool.get_conn()?),
            None => {
                let conn_str = config::get_mysql_conn_string();
                let pool = Pool::new(conn_str)?;
                let conn = pool.get_conn()?;
                CONNECTION = Some(pool);
                Ok(conn)
            }
        }
    }
}

pub fn create_database() -> Result<(), mysql::Error> {
    let mut conn = get_connection()?;
    conn.query_drop(r"
        create table if not exists Apps(
            id_app bigint not null auto_increment,
            name_app varchar(255) not null,
            id_user bigint not null,
            token_app varchar(255) not null unique,
            webhook_url_app varchar(255) not null,
            primary key (id_app)
        )
    ")?;

    conn.query_drop("create index if not exists name_app_index on Apps(name_app)")?;
    conn.query_drop("create index if not exists id_user_index on Apps(id_user)")?;

    conn.query_drop(r"
        create table if not exists Reports(
            id_report bigint not null auto_increment,
            severity_report int not null,
            title_report varchar(255) not null,
            message_report text not null,
            stacktrace_report longtext,
            timestamp_report bigint not null,
            app_report bigint not null,
            foreign key (app_report) references Apps(id_app),
            primary key (id_report)
        );
    ")?;

    conn.query_drop("create index if not exists title_report_index on Reports(title_report)")?;

    Ok(())
}
