#[path = "utilities.rs"]
mod utilities;

#[tokio::test]
async fn tallinn() -> anyhow::Result<()> {
    utilities::check_front_page(&wolt::get_front_page("tallinn").await?)
}

#[tokio::test]
async fn tartu() -> anyhow::Result<()> {
    utilities::check_front_page(&wolt::get_front_page("tartu").await?)
}
