use std::usize;

use serde::Serialize;

pub fn reduce_to_field(el: &str, cut_len: usize) -> String {
    if el.len() > cut_len {
        format!("{}...", &el[..cut_len - 3])
    } else {
        el.to_string()
    }
}

#[derive(Serialize)]
pub struct WebhookEmbed {
    pub embeds: Vec<EmbedSerializer>
}

#[derive(Serialize)]
pub struct EmbedSerializer {
    pub author: AuthorEmbed,
    pub title: String,
    pub description: String,
    pub color: u32,
    pub fields: Option<Vec<FieldEmbed>>
}

#[derive(Serialize)]
pub struct AuthorEmbed {
    pub name: String
}

#[derive(Serialize)]
pub struct FieldEmbed {
    pub name: String,
    pub value: String,
    pub inline: bool
}
