use reqwasm::http::{Request, RequestCredentials};
use serde::{Deserialize, Serialize};
use serde_json::json;

const BASE_URL: &str = "http://localhost:8080/api/v1";

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub status: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

pub async fn login_user(username: String, password: String) -> Result<UserLoginResponse, String> {
    let response = match Request::post(&format!("{}/auth/login", BASE_URL))
        .header("Content-Type", "application/json")
        .credentials(RequestCredentials::Include)
        .body(
            json!({
                "username": username,
                "password": password
            })
            .to_string(),
        )
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => {
            return Err("Network Error!".to_string());
        }
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("Network Error: {}", response.status()));
        }
    }

    let res_json = response.json::<UserLoginResponse>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}
