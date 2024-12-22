/// A struct that combines a query and an input into a single struct.
///
/// This is for the purpose of passing a complete set of HTTP request data to the handler,
/// allowing the freedom to design a REST structure that fits the application's needs.
pub struct CombinedInput<Q, I>
where
    Q: serde::de::DeserializeOwned,
    I: serde::de::DeserializeOwned,
{
    pub query: Q,
    pub input: Option<I>,
}
