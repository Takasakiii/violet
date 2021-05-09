use std::env;

pub fn get_mysql_conn_string() -> String {
    env::var("VIOLET_MYSQL_CONN")
        .expect("Esperado `VIOLET_MYSQL_CONN` nas enviroments")
}


pub fn get_discord_token() -> String {
    env::var("VIOLET_DISCORD_TOKEN")
        .expect("Esperado `VIOLET_DISCORD_TOKEN` nas enviroments")
}
