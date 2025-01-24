#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: String,
}

impl Ctx {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
        }
    }
}
