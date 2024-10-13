use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use js_sys::{Error, JsString};
use serde::{Deserialize, Serialize};
use web_sys::console;

mod utils;

#[derive(Serialize, Deserialize)]
struct ApiPayload {
    slack_code: Option<String>,
    github_code: Option<String>,
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
        Ok(response_text)
    } else {
        Err(JsValue::from_str(&format!("Request failed with status: {}", status)))
    }
}