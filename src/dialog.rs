#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DialogItem {
    pub text: String,
    pub response: String,
}