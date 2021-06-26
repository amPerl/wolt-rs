fn test_venue_results(venue_results: wolt::VenueResults) -> anyhow::Result<()> {
    let extra_keys = venue_results._extra_fields.keys().collect::<Vec<&String>>();
    assert_eq!(extra_keys, Vec::<&String>::new());

    assert_eq!(venue_results.results.len(), 1);

    let venue = venue_results.results.first().unwrap();

    dbg!(&venue.active_menu);

    let extra_keys = venue._extra_fields.keys().collect::<Vec<&String>>();
    assert_eq!(extra_keys, Vec::<&String>::new());

    let delivery_specs = &venue.delivery_specs;

    let extra_keys = delivery_specs
        ._extra_fields
        .keys()
        .collect::<Vec<&String>>();
    assert_eq!(extra_keys, Vec::<&String>::new());

    let delivery_pricing = &delivery_specs.delivery_pricing;

    let extra_keys = delivery_pricing
        ._extra_fields
        .keys()
        .collect::<Vec<&String>>();
    assert_eq!(extra_keys, Vec::<&String>::new());

    Ok(())
}

#[tokio::test]
async fn tallinn_mcdonalds_viru() -> anyhow::Result<()> {
    Ok(test_venue_results(
        wolt::get_venue("5af0707b3fe943000cb8f452").await?,
    )?)
}

#[tokio::test]
async fn tallinn_818() -> anyhow::Result<()> {
    Ok(test_venue_results(
        wolt::get_venue("5ff46a7022782e50a284da1b").await?,
    )?)
}

#[tokio::test]
async fn tartu_kauss_kampus() -> anyhow::Result<()> {
    Ok(test_venue_results(
        wolt::get_venue("5cca9fa7e44b4dfdc403b7c2").await?,
    )?)
}
