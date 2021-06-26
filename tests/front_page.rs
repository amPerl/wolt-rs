fn check_front_page(front_page: wolt::Page) -> anyhow::Result<()> {
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
    Ok(check_front_page(wolt::get_front_page("tallinn").await?)?)
}

#[tokio::test]
async fn tartu() -> anyhow::Result<()> {
    Ok(check_front_page(wolt::get_front_page("tartu").await?)?)
}
