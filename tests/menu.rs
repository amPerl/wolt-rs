#[path = "utilities.rs"]
mod utilities;

#[tokio::test]
async fn tallinn_mcdonalds_viru() -> anyhow::Result<()> {
    utilities::test_menu_results(&wolt::get_menu("5af075fd1198ae000ce50ced").await?)
}

#[tokio::test]
async fn tallinn_818() -> anyhow::Result<()> {
    utilities::test_menu_results(&wolt::get_menu("5ff58b74421e9ce09bacb9bb").await?)
}

#[tokio::test]
async fn tartu_kauss_kampus() -> anyhow::Result<()> {
    utilities::test_menu_results(&wolt::get_menu("5ccaa02f88269fabc47961aa").await?)
}
