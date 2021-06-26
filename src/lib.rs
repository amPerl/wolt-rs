use anyhow::Context;

mod model;
pub use model::*;

pub async fn get_front_page(city: &str) -> anyhow::Result<Page> {
    let url = format!("https://restaurant-api.wolt.com/v1/pages/front/{}", city);
    Ok(reqwest::get(url).await?.json().await?)
}

pub async fn get_delivery_page(lat: f64, lon: f64) -> anyhow::Result<Page> {
    let url = format!(
        "https://restaurant-api.wolt.com/v1/pages/delivery?lat={}&lon={}",
        lat, lon
    );
    Ok(reqwest::get(url).await?.json().await?)
}

pub async fn get_menu(oid: &str) -> anyhow::Result<MenuResults> {
    let url = format!("https://restaurant-api.wolt.com/v3/menus/{}", oid);
    Ok(reqwest::get(url).await?.json().await?)
}

pub async fn get_venue(id: &str) -> anyhow::Result<VenueResults> {
    let url = format!("https://restaurant-api.wolt.com/v3/venues/{}", id);
    Ok(reqwest::get(url)
        .await
        .context("failed to fetch venue")?
        .json()
        .await
        .context("failed to parse venue")?)
}

pub async fn get_venue_estimates(id: &str) -> anyhow::Result<EstimateResults> {
    let url = format!("https://restaurant-api.wolt.com/v1/venues/{}/estimates", id);
    Ok(reqwest::Client::new()
        .post(url)
        .json(&std::collections::HashMap::<String, String>::new())
        .send()
        .await
        .context("failed to fetch venue estimates")?
        .json()
        .await
        .context("failed to parse venue estimates")?)
}
