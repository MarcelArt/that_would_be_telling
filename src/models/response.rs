use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub is_success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn success(data: Option<T>, message: String) -> Self {
        Response {
            is_success: true,
            message,
            data,
        }
    }

    pub fn failure(message: String) -> Self {
        Response {
            is_success: false,
            message,
            data: None,
        }
    }
}