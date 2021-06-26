use super::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Category {
    pub id: ObjectReference,
    pub _id: ObjectReference,
    pub name: Vec<LocalizedString>,
    pub description: Vec<LocalizedString>,

    #[serde(flatten)]
    pub _extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct MenuItemOption {
    pub id: ObjectReference,
    pub _id: ObjectReference,
    pub parent: ObjectReference,
    pub name: Vec<LocalizedString>,
    pub free_selections: i32,
    pub maximum_single_selections: i32,
    pub maximum_total_selections: i32,
    pub minimum_total_selections: i32,
    pub required_option_selections: Vec<ObjectReference>,

    #[serde(flatten)]
    pub _extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct MenuItemTagStyle {
    pub text_color: String,
    pub background_color: String,
    pub decoration: String,

    #[serde(flatten)]
    pub _extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct MenuItemTag {
    pub id: String,
    pub name: Vec<LocalizedString>,
    pub style: MenuItemTagStyle,

    #[serde(flatten)]
    pub _extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct MenuItemTime {
    pub available_days_of_week: Vec<i32>,
    pub available_start_date: Date,
    pub available_end_date: Date,
    pub visible_days_of_week: Vec<i32>,
    pub visible_start_date: Date,
    pub visible_end_date: Date,

    #[serde(flatten)]
    pub _extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct MenuItemValidity {
    pub start_date: Date,

    #[serde(flatten)]
    pub _extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct MenuItem {
    pub id: ObjectReference,
    pub _id: ObjectReference,
    pub category: ObjectReference,
    pub enabled: bool,
    pub name: Vec<LocalizedString>,
    pub description: Vec<LocalizedString>,
    pub alcohol_percentage: i32,
    pub baseprice: i32,
    pub vat_percentage: i32,
    pub allowed_delivery_methods: Vec<String>,
    pub r#type: String,
    pub ftu_restrictions: Vec<String>,
    pub has_extra_info: bool,
    pub image: Option<String>,
    pub image_blurhash: Option<String>,
    pub checksum: String,
    pub is_venue_tip: bool,
    pub options: Vec<MenuItemOption>,
    pub tags: Vec<MenuItemTag>,
    pub times: Vec<MenuItemTime>,
    pub allowed_order_types: Option<Vec<String>>,
    pub original_price: Option<i32>,
    pub validity: Option<MenuItemValidity>,

    #[serde(flatten)]
    pub _extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct Recommendation {
    pub item: ObjectReference,
    pub options: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct MenuLanguage {
    pub autotranslated: bool,
    pub language: String,
    pub name: String,
    pub primary: bool,
}

#[derive(Deserialize, Debug)]
pub struct MenuOptionValue {
    pub id: ObjectReference,
    pub price: i32,
    pub name: Vec<LocalizedString>,
    pub minimum_selections: i32,
    pub maximum_selections: i32,
    pub hide_when_selected: bool,
    pub multichoice_weight: i32,

    #[serde(flatten)]
    pub _extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct MenuOption {
    pub id: ObjectReference,
    pub _id: ObjectReference,
    pub defaultvalue: ObjectReference,
    pub r#type: String,
    pub name: Vec<LocalizedString>,
    pub values: Vec<MenuOptionValue>,
    pub has_extra_info: bool,

    #[serde(flatten)]
    pub _extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct Menu {
    pub id: ObjectReference,
    pub _id: ObjectReference,
    pub itemid: ObjectReference,
    pub merchant: ObjectReference,
    pub name: Vec<LocalizedString>,
    pub categories: Vec<Category>,
    pub items: Vec<MenuItem>,
    pub recommendations: Vec<Recommendation>,
    pub language: String,
    pub languages: Vec<MenuLanguage>,
    pub options: Vec<MenuOption>,
    #[serde(flatten)]
    pub _extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct MenuResults {
    pub results: Vec<Menu>,

    #[serde(flatten)]
    pub _extra_fields: std::collections::HashMap<String, serde_json::Value>,
}
