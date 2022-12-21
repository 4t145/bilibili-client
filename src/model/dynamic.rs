use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicCard {
    pub item: Item,
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub at_control: String,
    pub category: String,
    pub description: String,
    pub id: i64,
    pub is_fav: i64,
    pub pictures: Vec<Pictures>,
    pub pictures_count: i64,
    pub reply: i64,
    pub role: Vec<Value>,
    pub settings: Settings,
    pub source: Vec<Value>,
    pub title: String,
    pub upload_time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pictures {
    pub img_height: i64,
    pub img_size: f64,
    pub img_src: String,
    pub img_tags: Value,
    pub img_width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub copy_forbidden: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub head_url: String,
    pub name: String,
    pub uid: i64,
    pub vip: Vip,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vip {
    pub avatar_subscript: i64,
    pub due_date: i64,
    pub label: Label,
    pub nickname_color: String,
    pub status: i64,
    pub theme_type: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub vip_pay_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Label {
    pub label_theme: String,
    pub path: String,
    pub text: String,
}