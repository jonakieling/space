#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DialogItem {
    pub text: String,
    pub response: String,
    pub action: Option<DialogAction>
}

impl ToString for DialogItem {
    fn to_string(&self) -> String {
        self.text.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DialogAction {
    Trade
}