use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Data<T> {
    pub success: bool,
    pub data: T,
    pub errors: Vec<String>,
}
