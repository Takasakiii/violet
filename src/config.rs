use std::env;

pub fn get_mysql_conn_string() -> String {
    env::var("VIOLET_MYSQL_CONN")
        .expect("Esperado `VIOLET_MYSQL_CONN` nas enviroments")
}


pub fn get_discord_token() -> String {
    env::var("VIOLET_DISCORD_TOKEN")
        .expect("Esperado `VIOLET_DISCORD_TOKEN` nas enviroments")
}

pub fn get_bot_prefix() -> String {
    env::var("VIOLET_BOT_PREFIX")
        .unwrap_or_else(|_| "v.".into())
}

pub fn get_jwt_secret() -> String {
    env::var("VIOLET_JWT_SECRET")
        .expect("Esperado `VIOLET_JWT_SECRET` nas enviroments")
}

pub fn get_bot_owner() -> u64 {
    env::var("VIOLET_ID_OWNER")
        .unwrap_or_else(|_| "274289097689006080".into())
        .parse::<u64>()
        .expect("O enviroment `VIOLET_ID_OWNER` não é um numero.")
}
