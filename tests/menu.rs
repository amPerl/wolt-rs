fn test_menu_results(menu_results: wolt::MenuResults) -> anyhow::Result<()> {
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

#[tokio::test]
async fn tallinn_mcdonalds_viru() -> anyhow::Result<()> {
    Ok(test_menu_results(
        wolt::get_menu("5af075fd1198ae000ce50ced").await?,
    )?)
}

#[tokio::test]
async fn tallinn_818() -> anyhow::Result<()> {
    Ok(test_menu_results(
        wolt::get_menu("5ff58b74421e9ce09bacb9bb").await?,
    )?)
}

#[tokio::test]
async fn tartu_kauss_kampus() -> anyhow::Result<()> {
    Ok(test_menu_results(
        wolt::get_menu("5ccaa02f88269fabc47961aa").await?,
    )?)
}
