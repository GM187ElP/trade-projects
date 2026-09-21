use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignInRequset {
    pub email: String,
    pub password: String,
}
