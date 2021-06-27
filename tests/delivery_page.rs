#[path = "utilities.rs"]
mod utilities;

#[tokio::test]
async fn tallinn() -> anyhow::Result<()> {
    utilities::check_delivery_page(&wolt::get_delivery_page(59.43, 24.75).await?)
}

#[tokio::test]
async fn tartu() -> anyhow::Result<()> {
    utilities::check_delivery_page(&wolt::get_delivery_page(58.378, 26.729).await?)
}
