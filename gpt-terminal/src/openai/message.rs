use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Model {
    name: String,
    max_token: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
}

impl Message {
    pub fn new(content: &str) -> Self {
        Self {
            role: MessageRole::User,
            content: content.to_string(),
        }
    }
    pub fn new_assistant(content: &str) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: content.to_string(),
        }
    }
    pub fn new_default_system() -> Self {
        Self {
            role: MessageRole::System,
            content: "You are a helpful assistant.".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    System,
    Assistant,
    User,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        assert_eq!(
            serde_json::to_string(&Message::new("Hello World")).unwrap(),
            "{\"role\":\"user\",\"content\":\"Hello World\"}"
        )
    }
}
