use awc;
use awc::error::{SendRequestError};

const API_URL: &str = "https://www.nordnet.fi/";

pub async fn get_accounts() -> Result<i32, SendRequestError> {
    dbg!("requesting");
    let client = awc::Client::new();
    let accounts = client.get(format!("{}/api/accounts", API_URL))
        .send()
        .await?;

    dbg!(accounts);
    Ok(0)
}