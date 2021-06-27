fn test_venue_results(venue_results: wolt::VenueResults) -> anyhow::Result<()> {
    let extra_keys = venue_results._extra_fields.keys().collect::<Vec<&String>>();
    assert_eq!(extra_keys, Vec::<&String>::new());

    assert_eq!(venue_results.status, "OK");
    assert!(venue_results.results.is_some());

    let results = venue_results.results.unwrap();

    assert_eq!(results.len(), 1);

    let venue = results.first().unwrap();

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

#[cfg(feature = "long_test")]
async fn test_delivery_page(page: wolt::Page) -> anyhow::Result<()> {
    match page {
        wolt::Page::Delivery { sections, .. } => {
            for section in sections {
                if let Some(section_items) = section.items {
                    for section_item in section_items {
                        if let Some(section_item_venue) = section_item.venue {
                            let wolt::PageSectionItemVenue { name, id, .. } = section_item_venue;
                            dbg!(&name, &id);

                            let venue_results = wolt::get_venue(&id).await?;
                            test_venue_results(venue_results)?;
                        }
                    }
                }
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}

#[cfg(feature = "long_test")]
#[tokio::test]
async fn tallinn_venues() -> anyhow::Result<()> {
    let tallinn_delivery_page = wolt::get_delivery_page(59.43, 24.75).await?;
    test_delivery_page(tallinn_delivery_page).await?;
    Ok(())
}

#[cfg(feature = "long_test")]
#[tokio::test]
async fn tartu_venues() -> anyhow::Result<()> {
    let tartu_delivery_page = wolt::get_delivery_page(58.378, 26.729).await?;
    test_delivery_page(tartu_delivery_page).await?;
    Ok(())
}
