use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub user_id: i32,
    pub group_id: Option<i32>,
}
