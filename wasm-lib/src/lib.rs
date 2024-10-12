mod utils;

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use wasm_bindgen_futures::future_to_promise;
use js_sys::Promise;
use url::Url;

#[derive(Serialize, Deserialize)]
struct ApiPayload {
    slack_code: Option<String>,
    github_code: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SlackResponse {
    hashed_secret: String,
    slack_id: String,
    eligibility: String,
    username: String,
}

#[derive(Serialize, Deserialize)]
struct GitHubResponse {
    name: String,
    id: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    Slack: SlackResponse,
    GitHub: GitHubResponse,
}

#[wasm_bindgen]
pub fn verify_api(slack_code: Option<String>, github_code: Option<String>) -> Promise {
    let payload = ApiPayload {
        slack_code,
        github_code,
    };

    let client = Client::new();
    let request = client.post("https://api.onboard.limeskey.com")
        .json(&payload)
        .send();

    future_to_promise(async move {
        match request.await {
            Ok(response) => {
                let payload_json = serde_json::to_string(&payload).unwrap_or_else(|_| "{}".to_string());
                let status = response.status();
                let response_text = response.text().await.unwrap_or_else(|_| "Failed to read response".to_string());

                if status.is_success() {
                    let api_response: ApiResponse = serde_json::from_str(&response_text).unwrap();

                    // Generate the URL with appended parameters
                    let mut url = Url::parse("https://forms.hackclub.com/t/9yNy4WYtrZus").unwrap();
                    url.query_pairs_mut()
                        .append_pair("secret", &api_response.Slack.hashed_secret)
                        .append_pair("slack_id", &api_response.Slack.slack_id)
                        .append_pair("eligibility", &api_response.Slack.eligibility)
                        .append_pair("slack_user", &api_response.Slack.username)
                        .append_pair("github_id", &api_response.GitHub.id);

                    Ok(JsValue::from_str(&url.to_string()))
                } else {
                    Err(JsValue::from_str(&format!("Request failed: {}\nResponse: {}", payload_json, response_text)))
                }
            },
            Err(_) => {
                let payload_json = serde_json::to_string(&payload).unwrap_or_else(|_| "{}".to_string());
                Err(JsValue::from_str(&format!("Request error: {}", payload_json)))
            },
        }
    })
}