#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DialogItem {
    pub text: String,
    pub response: String,
    pub action: Option<DialogAction>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DialogAction {
    Trade
}