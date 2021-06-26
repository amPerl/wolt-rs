use serde::Deserialize;

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
