use super::models;
use chrono::Datelike;
use coffeeshop::{
    prelude::{CoffeeMachineError, Machine, QueryType, ValidationError},
    reexports::{async_trait, axum},
};
use std::sync::atomic::{AtomicUsize, Ordering};

/// A simple machine that says hello.
#[derive(Debug, Default)]
pub struct HelloMachine {
    /// The total amount of historical requests processed.
    pub process_count: AtomicUsize,
}

#[async_trait]
impl Machine<models::HelloQuery, models::HelloPayload, models::HelloResult> for HelloMachine {
    /// Required method for the [`Machine`] trait.
    ///
    /// A [`Machine`] is expected to process the input and return the output; if an error
    /// occurs, it should return a [`CoffeeMachineError`].
    async fn call(
        &self,
        query: &models::HelloQuery,
        input: Option<&models::HelloPayload>,
    ) -> Result<models::HelloResult, CoffeeMachineError> {
        let year = chrono::Utc::now()
            .year()
            .saturating_sub(input.unwrap().age as i32);

        let name = input.unwrap().name.as_str();

        // Arbitrary error to show how to return an error.
        if name.to_ascii_lowercase() == "little timmy" {
            return Err(CoffeeMachineError::new(
                axum::http::StatusCode::FORBIDDEN,
                "ForbiddenUser".to_owned(),
                Some(serde_json::json!({
                    "message": "Little Timmy is not allowed to use this system.",
                })),
            ));
        }

        let greeting = format!(
            "{greeting}, {name}! {year} is a good year to be born in.",
            greeting = query.language.greeting(),
            name = input.unwrap().name,
            year = year,
        );

        self.process_count.fetch_add(1, Ordering::Relaxed);

        // Simulate a long-running process.
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        Ok(models::HelloResult {
            greeting,
            answer_id: self.process_count.load(Ordering::Relaxed),
        })
    }

    /// Validate the input before processing.
    ///
    /// This prevents erroronous input from being sent to the SQS in the first place;
    /// and the [`Waiter`] will return a [`http::StatusCode::UNPROCESSABLE_ENTITY`] response
    /// with the given [`ValidationError`] as [details](serde_json::Value).
    async fn validator(
        &self,
        query: &models::HelloQuery,
        input: Option<&models::HelloPayload>,
    ) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();

        if input.is_none() {
            errors.insert("$body".to_owned(), "The input is missing.".to_owned());
        }

        let payload = input.unwrap();

        match payload.age {
            0 => {
                errors.insert("age".to_owned(), "Age must be positive.".to_owned());
            }
            1..=17 => {
                errors.insert(
                    "age".to_owned(),
                    "You must be 18 years or older to use this service.".to_owned(),
                );
            }
            i if i >= 130 => {
                errors.insert(
                    "age".to_owned(),
                    "I don't think you are truthful about your age.".to_owned(),
                );
            }
            _ => {}
        }

        if query.get_timeout() < Some(tokio::time::Duration::from_secs(1)) {
            errors.insert(
                "timeout".to_owned(),
                "The timeout must be at least 1 second.".to_owned(),
            );
        }

        // If there are no errors, return `Ok(())`.
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
