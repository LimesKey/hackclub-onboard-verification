use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use js_sys::{Error, JsString};
use serde::{Deserialize, Serialize};
use web_sys::console;
use url::Url;

mod utils;

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
    slack: SlackResponse,
    github: GitHubResponse,
}

#[wasm_bindgen()]
pub async fn verify_api(slack_code: Option<String>, github_code: Option<String>) -> Result<JsValue, JsValue> {
    utils::set_panic_hook();

    // Create the JSON payload
    let payload = ApiPayload {
        slack_code,
        github_code,
    };

    let payload_json = serde_json::to_string(&payload).unwrap();

    // Initialize the POST request
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_body(&JsValue::from(JsString::from(payload_json)));
    opts.set_mode(RequestMode::Cors);  // Allow cross-origin requests
    let headers = web_sys::Headers::new().unwrap();
    headers.set("Content-Type", "application/json").unwrap();
    opts.set_headers(&headers);

    // Create the request object with the API URL
    let request = Request::new_with_str_and_init("https://api.onboard.limeskey.com/api", &opts)
        .map_err(|e| JsValue::from(Error::new(&format!("Request creation failed: {:?}", e))))?;

    // Fetch the request
    let window = web_sys::window().unwrap();
    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // Convert the response to a `Response` object
    let response: Response = response_value.dyn_into().unwrap();
    let status = response.status();
    let response_text = JsFuture::from(response.text()?).await?;

    console::log_2(&"Response status:".into(), &status.into());
    console::log_1(&response_text);

    if status == 200 {
        let api_response: ApiResponse = serde_json::from_str(&response_text.as_string().unwrap()).unwrap();

        // Generate the URL with appended parameters
        let mut url = Url::parse("https://forms.hackclub.com/t/9yNy4WYtrZus").unwrap();
        url.query_pairs_mut()
            .append_pair("secret", &api_response.slack.hashed_secret)
            .append_pair("slack_id", &api_response.slack.slack_id)
            .append_pair("eligibility", &api_response.slack.eligibility)
            .append_pair("slack_user", &api_response.slack.username)
            .append_pair("github_id", &api_response.github.id);

        console::log_1(&"Successfully generated URL".into());
        Ok(JsValue::from_str(&url.to_string()))
    } else {
        Err(JsValue::from_str(&format!("Request failed with status: {}", status)))
    }
}