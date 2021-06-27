#[path = "utilities.rs"]
mod utilities;

#[tokio::test]
async fn tallinn_mcdonalds_viru() -> anyhow::Result<()> {
    utilities::test_venue_results(&wolt::get_venue("5af0707b3fe943000cb8f452").await?)
}

#[tokio::test]
async fn tallinn_818() -> anyhow::Result<()> {
    utilities::test_venue_results(&wolt::get_venue("5ff46a7022782e50a284da1b").await?)
}

#[tokio::test]
async fn tartu_kauss_kampus() -> anyhow::Result<()> {
    utilities::test_venue_results(&wolt::get_venue("5cca9fa7e44b4dfdc403b7c2").await?)
}
