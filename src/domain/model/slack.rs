use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Slack {
    pub blocks: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Message {
    pub r#type: String,
    pub text: Text,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Text {
    pub r#type: String,
    pub text: String,
}
