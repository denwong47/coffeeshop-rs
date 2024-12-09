use coffeeshop::prelude::*;

use serde::{Deserialize, Serialize};

/// Languages supported by the hello world service.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum AcceptedLanguage {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "es")]
    Italian,
    #[serde(rename = "zh")]
    Chinese,
}

impl Default for AcceptedLanguage {
    fn default() -> Self {
        Self::English
    }
}

impl AcceptedLanguage {
    /// Get the greeting message for the language.
    pub fn greeting(&self) -> &'static str {
        match self {
            Self::English => "Hello",
            Self::Italian => "Ciao",
            Self::Chinese => "你好",
        }
    }
}

#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct HelloQuery {
    pub language: AcceptedLanguage,
    #[serde_as(as = "Option<serde_with::DurationSecondsWithFrac<f64>>")]
    pub timeout: Option<tokio::time::Duration>,
    #[serde(rename = "async")]
    #[serde(default)]
    pub is_async: bool,
}

impl QueryType for HelloQuery {
    /// Let the [`Waiter`] know how long to wait for the response before timing out.
    fn get_timeout(&self) -> Option<tokio::time::Duration> {
        self.timeout
    }

    /// Let the [`Waiter`] know if the request is asynchronous.
    fn is_async(&self) -> bool {
        self.is_async
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct HelloPayload {
    pub name: String,
    pub age: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct HelloResult {
    pub greeting: String,
    pub answer_id: usize,
}
