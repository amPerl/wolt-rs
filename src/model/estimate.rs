use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EstimateBounds {
    pub mean: i32,
    pub max: Option<i32>,
    pub min: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct Estimates {
    pub delivery: EstimateBounds,
    pub pickup: EstimateBounds,
    pub preparation: EstimateBounds,
    pub total: EstimateBounds,
}

#[derive(Deserialize, Debug)]
pub struct EstimateResults {
    pub status: String,
    pub results: Option<Estimates>,
}
