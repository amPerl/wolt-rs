use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PageCategory {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct ConfigurationDebug {
    pub a: i32,
    pub b: i32,
}
