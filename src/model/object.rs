use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ObjectReference {
    #[serde(rename = "$oid")]
    pub oid: String,
}
