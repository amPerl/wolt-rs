#[allow(dead_code)]
pub fn check_front_page(front_page: &wolt::Page) -> anyhow::Result<()> {
    match front_page {
        wolt::Page::Front {
            _extra_fields,
            sections,
            ..
        } => {
            let extra_keys = _extra_fields.keys().collect::<Vec<&String>>();
            assert_eq!(extra_keys, Vec::<&String>::new());

            assert_ne!(sections.len(), 0);
            for section in sections {
                let extra_keys = section._extra_fields.keys().collect::<Vec<&String>>();
                assert_eq!(extra_keys, Vec::<&String>::new());

                if let Some(section_items) = &section.items {
                    assert_ne!(section_items.len(), 0);
                    for item in section_items {
                        let extra_keys = item._extra_fields.keys().collect::<Vec<&String>>();
                        assert_eq!(extra_keys, Vec::<&String>::new());
                    }
                }
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}

#[allow(dead_code)]
pub fn check_delivery_page(delivery_page: &wolt::Page) -> anyhow::Result<()> {
    match delivery_page {
        wolt::Page::Delivery {
            _extra_fields,
            sections,
            ..
        } => {
            let extra_keys = _extra_fields.keys().collect::<Vec<&String>>();
            assert_eq!(extra_keys, Vec::<&String>::new());

            assert_ne!(sections.len(), 0);
            for section in sections {
                let extra_keys = section._extra_fields.keys().collect::<Vec<&String>>();
                assert_eq!(extra_keys, Vec::<&String>::new());

                if let Some(section_items) = &section.items {
                    assert_ne!(section_items.len(), 0);
                    for item in section_items {
                        let extra_keys = item._extra_fields.keys().collect::<Vec<&String>>();
                        assert_eq!(extra_keys, Vec::<&String>::new());
                    }
                }
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}

#[allow(dead_code)]
pub fn test_menu_results(menu_results: &wolt::MenuResults) -> anyhow::Result<()> {
    let extra_keys = menu_results._extra_fields.keys().collect::<Vec<&String>>();
    assert_eq!(extra_keys, Vec::<&String>::new());

    assert_eq!(menu_results.results.len(), 1);

    let menu = menu_results.results.first().unwrap();

    let extra_keys = menu._extra_fields.keys().collect::<Vec<&String>>();
    assert_eq!(extra_keys, Vec::<&String>::new());

    for option in menu.options.iter() {
        let extra_keys = option._extra_fields.keys().collect::<Vec<&String>>();
        assert_eq!(extra_keys, Vec::<&String>::new());

        for value in option.values.iter() {
            let extra_keys = value._extra_fields.keys().collect::<Vec<&String>>();
            assert_eq!(extra_keys, Vec::<&String>::new());
        }
    }

    for category in menu.categories.iter() {
        let extra_keys = category._extra_fields.keys().collect::<Vec<&String>>();
        assert_eq!(extra_keys, Vec::<&String>::new());
    }

    for menu_item in menu.items.iter() {
        let extra_keys = menu_item._extra_fields.keys().collect::<Vec<&String>>();
        assert_eq!(extra_keys, Vec::<&String>::new());

        if let Some(validity) = &menu_item.validity {
            let extra_keys = validity._extra_fields.keys().collect::<Vec<&String>>();
            assert_eq!(extra_keys, Vec::<&String>::new());
        }

        for option in menu_item.options.iter() {
            let extra_keys = option._extra_fields.keys().collect::<Vec<&String>>();
            assert_eq!(extra_keys, Vec::<&String>::new());
        }

        for tag in menu_item.tags.iter() {
            let extra_keys = tag._extra_fields.keys().collect::<Vec<&String>>();
            assert_eq!(extra_keys, Vec::<&String>::new());

            let extra_keys = tag.style._extra_fields.keys().collect::<Vec<&String>>();
            assert_eq!(extra_keys, Vec::<&String>::new());
        }

        for time in menu_item.times.iter() {
            let extra_keys = time._extra_fields.keys().collect::<Vec<&String>>();
            assert_eq!(extra_keys, Vec::<&String>::new());
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub fn test_venue_results(venue_results: &wolt::VenueResults) -> anyhow::Result<()> {
    let extra_keys = venue_results._extra_fields.keys().collect::<Vec<&String>>();
    assert_eq!(extra_keys, Vec::<&String>::new());

    assert_eq!(venue_results.status, "OK");
    assert!(venue_results.results.is_some());

    let results = venue_results.results.as_ref().unwrap();

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
