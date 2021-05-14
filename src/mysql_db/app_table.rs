use mysql::{params, prelude::Queryable};

use crate::tokens::Tokens;

pub struct AppTable {
    pub id: u64,
    pub name: String,
    pub id_user: u64,
    pub token_app: String,
    pub webhook_url: String
}

impl AppTable {
    pub fn insert(name: &String, id_user: u64, webhook_url: &String) -> Result<Self, serenity::framework::standard::CommandError> {
        let token_builder = match Tokens::new() {
            Ok(token_builder) => token_builder,
            Err(why) => return Err(format!("{:?}", why).into())
        };

        let token = token_builder.generate_token(id_user)?;

        let mut conn = super::get_connection()?;
        conn.exec_drop(r"
            insert into Apps(name_app, id_user, token_app, webhook_url_app) values (:name, :user, :token, :webhook)
        ", params! {
            "name" => &name,
            "user" => id_user,
            "token" => &token,
            "webhook" => &webhook_url
        })?;

        let id = conn.last_insert_id();
        let result = Self {
            id,
            name: name.clone(),
            id_user,
            token_app: token,
            webhook_url: webhook_url.clone()
        };

        Ok(result)
    }
}
