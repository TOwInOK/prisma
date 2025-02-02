#[derive(Debug, Default, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct Options {
    pub freeze: bool,
    pub force_update: bool,
}
