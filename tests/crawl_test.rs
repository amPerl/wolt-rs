#[path = "utilities.rs"]
mod utilities;

#[cfg(feature = "crawl_test")]
async fn crawl_delivery_page(page: wolt::Page) -> anyhow::Result<()> {
    match page {
        wolt::Page::Delivery { sections, .. } => {
            for section in sections {
                if let Some(section_items) = section.items {
                    for section_item in section_items {
                        if let Some(section_item_venue) = section_item.venue {
                            let wolt::PageSectionItemVenue { name, id, .. } = section_item_venue;
                            dbg!(&name, &id);

                            let venue_results = wolt::get_venue(&id).await?;
                            utilities::test_venue_results(&venue_results)?;

                            let venues = venue_results.results.as_ref().unwrap();
                            let venue = venues.first().unwrap();

                            let menu_results = wolt::get_menu(&venue.active_menu.oid).await?;
                            utilities::test_menu_results(&menu_results)?;
                        }
                    }
                }
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}

#[cfg(feature = "crawl_test")]
#[tokio::test]
async fn tallinn_venues() -> anyhow::Result<()> {
    let tallinn_delivery_page = wolt::get_delivery_page(59.43, 24.75).await?;
    crawl_delivery_page(tallinn_delivery_page).await?;
    Ok(())
}

#[cfg(feature = "crawl_test")]
#[tokio::test]
async fn tartu_venues() -> anyhow::Result<()> {
    let tartu_delivery_page = wolt::get_delivery_page(58.378, 26.729).await?;
    crawl_delivery_page(tartu_delivery_page).await?;
    Ok(())
}
