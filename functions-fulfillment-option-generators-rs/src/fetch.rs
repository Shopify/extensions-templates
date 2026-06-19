use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function]
fn fetch(input: schema::fetch::Input) -> Result<schema::FunctionFetchResult> {
    // Only reach out to the external service when it can return relevant
    // fulfillment options. Returning no request skips the network call entirely.
    if input.localization().country().iso_code().as_str() == "CA" {
        return Ok(schema::FunctionFetchResult {
            request: Some(build_external_api_request()),
        });
    }

    Ok(schema::FunctionFetchResult { request: None })
}

fn build_external_api_request() -> schema::HttpRequest {
    schema::HttpRequest {
        method: schema::HttpRequestMethod::Get,
        url: "https://cdn.shopify.com/s/files/1/0628/3830/9033/files/pickup-points-external-api-v2.json?v=1714588690".to_string(),
        headers: vec![schema::HttpRequestHeader {
            name: "Accept".to_string(),
            value: "application/json; charset=utf-8".to_string(),
        }],
        body: None,
        json_body: None,
        policy: schema::HttpRequestPolicy {
            read_timeout_ms: 500,
        },
    }
}
