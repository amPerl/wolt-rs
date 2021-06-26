use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Location {
    pub coordinates: (f64, f64),
}

#[derive(Deserialize, Debug)]
pub struct CityData {
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct Date {
    #[serde(rename = "$date")]
    pub timestamp: u64,
}

#[derive(Deserialize, Debug)]
pub struct LocalizedString {
    pub lang: String,
    pub value: String,
    pub verified: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Link {
    pub title: String,
    pub r#type: String,
    pub target: String,
    pub target_sort: String,
    pub target_title: String,
}

#[derive(Deserialize, Debug)]
pub struct Image {
    pub blurhash: Option<String>,
    pub url: String,
    pub variants: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Price {
    #[serde(rename = "relative-discount")]
    RelativeDiscount { discount_percentage: String },
    #[serde(rename = "absolute-discount")]
    AbsoluteDiscount { current: String, original: String },
}
