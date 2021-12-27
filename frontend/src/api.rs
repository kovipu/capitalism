use std::collections::HashMap;

use dotenv_codegen::dotenv;
use reqwest::Error;

const API_ROOT: &str = dotenv!("API_ROOT");

// exchange username and password for a jwt token
pub async fn login(username: &str, password: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let url = format!("{}/login", API_ROOT);
    let params = HashMap::from([("username", username), ("password", password)]);

    let res = client.post(url).form(&params).send().await?;

    Ok(res.text().await?)
}
