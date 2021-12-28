use dotenv_codegen::dotenv;
use reqwasm::http::{Request, Response};
use reqwasm::Error;

const API_ROOT: &str = dotenv!("API_ROOT");

// exchange username and password for a jwt token
pub async fn login(auth: &str) -> Result<Response, Error> {
    let req: Request = Request::post(&format!("{}/login", API_ROOT)).header("Authorization", auth);

    req.send().await
}

pub fn build_auth_header(username: &str, password: &str) -> String {
    format!(
        "Basic {}",
        base64::encode(format!("{}:{}", username, password))
    )
}
