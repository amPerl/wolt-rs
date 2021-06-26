fn check_delivery_page(delivery_page: wolt::Page) -> anyhow::Result<()> {
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

                assert_ne!(section.items.len(), 0);
                for item in section.items {
                    let extra_keys = item._extra_fields.keys().collect::<Vec<&String>>();
                    assert_eq!(extra_keys, Vec::<&String>::new());
                }
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}

#[tokio::test]
async fn tallinn() -> anyhow::Result<()> {
    Ok(check_delivery_page(
        wolt::get_delivery_page(59.43, 24.75).await?,
    )?)
}

#[tokio::test]
async fn tartu() -> anyhow::Result<()> {
    Ok(check_delivery_page(
        wolt::get_delivery_page(58.378, 26.729).await?,
    )?)
}
