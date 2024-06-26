use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTaskSchema {
    pub titlle: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppState {
    pub name: String,
}
