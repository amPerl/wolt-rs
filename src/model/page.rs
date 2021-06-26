use super::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigurationDebug {
    pub a: i32,
    pub b: i32,
}

#[derive(Deserialize, Debug)]
pub struct PageSectionItemVenueRating {
    pub rating: i32,
    pub score: f64,
}

#[derive(Deserialize, Debug)]
pub struct PageSectionItemVenue {
    pub id: String,
    pub name: String,
    pub delivery_price: String,
    pub rating: Option<PageSectionItemVenueRating>,
}

#[derive(Deserialize, Debug)]
pub struct PageSectionItem {
    pub template: String,
    pub title: String,
    pub track_id: String,
    pub link: Link,
    pub image: Option<Image>,

    pub category: Option<String>,
    pub description: Option<String>,
    pub quantity: Option<i32>,
    pub quantity_str: Option<String>,

    pub venue: Option<PageSectionItemVenue>,
    pub overlay: Option<String>,

    pub price: Option<Price>,

    #[serde(flatten)]
    pub _extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "image_type")]
pub enum PageSectionImage {
    #[serde(rename = "lottie")]
    Lottie {
        name: String,
        url: String,
        variants: Vec<String>,
    },
}

#[derive(Deserialize, Debug)]
pub struct PageSection {
    pub items: Option<Vec<PageSectionItem>>,
    pub link: Link,
    pub template: String,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<PageSectionImage>,
    pub title: Option<String>,

    #[serde(flatten)]
    pub _extra_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct PageCategory {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct PageSharedAttributes {
    pub page_title: String,
    pub expires_in_seconds: i32,
    pub categories: Vec<PageCategory>,
    pub created: Date,
    pub show_large_title: bool,
    pub show_map: bool,
    pub track_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "name")]
pub enum Page {
    #[serde(rename = "front")]
    Front {
        city: String,
        city_data: CityData,
        configuration_debug: ConfigurationDebug,
        sections: Vec<PageSection>,

        #[serde(flatten)]
        page: PageSharedAttributes,

        #[serde(flatten)]
        _extra_fields: std::collections::HashMap<String, serde_json::Value>,
    },
    #[serde(rename = "delivery")]
    Delivery {
        sections: Vec<PageSection>,

        #[serde(flatten)]
        page: PageSharedAttributes,

        #[serde(flatten)]
        _extra_fields: std::collections::HashMap<String, serde_json::Value>,
    },
}
