use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function]
fn fetch(input: schema::fetch::Input) -> Result<schema::FunctionFetchResult> {
    let delivery_address = &input.delivery_address();
    if let (Some(country_code), Some(longitude), Some(latitude)) = (
        &delivery_address.country_code(),
        &delivery_address.longitude(),
        &delivery_address.latitude(),
    ) {
        if country_code.as_str() == "CA" {
            return Ok(schema::FunctionFetchResult {
                request: Some(build_external_api_request(latitude, longitude)),
            });
        }
    }

    Ok(schema::FunctionFetchResult { request: None })
}

fn build_external_api_request(latitude: &f64, longitude: &f64) -> schema::HttpRequest {
    // The latitude and longitude parameters are included in the URL for demonstration purposes only. They do not influence the result.
    let url = format!(
        "https://cdn.shopify.com/s/files/1/0628/3830/9033/files/pickup-points-external-api-v2.json?v=1714588690&lat={latitude}&lon={longitude}",
    );

    schema::HttpRequest {
        method: schema::HttpRequestMethod::Get,
        url,
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

#[cfg(test)]
mod tests {
    use super::*;
    use shopify_function::{run_function_with_input, Result};

    #[test]
    fn test_fetch_returns_request_when_country_is_canada() -> Result<()> {
        let result = run_function_with_input(
            fetch,
            r#"
                {
                    "deliveryAddress": {
                        "countryCode": "CA",
                        "longitude": 43.70,
                        "latitude": -79.42
                    }
                }
            "#,
        )?;

        let expected = schema::FunctionFetchResult {
            request: Some(schema::HttpRequest {
                method: schema::HttpRequestMethod::Get,
                url: "https://cdn.shopify.com/s/files/1/0628/3830/9033/files/pickup-points-external-api-v2.json?v=1714588690&lat=-79.42&lon=43.7".to_string(),

                headers: vec![
                    schema::HttpRequestHeader {
                        name: "Accept".to_string(),
                        value: "application/json; charset=utf-8".to_string(),
                    },
                ],
                body: None,
                json_body: None,
                policy: schema::HttpRequestPolicy {
                    read_timeout_ms: 500,
                },
            }),
        };

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_fetch_returns_no_request_when_country_is_not_canada() -> Result<()> {
        let result = run_function_with_input(
            fetch,
            r#"
                {
                    "deliveryAddress": {
                        "countryCode": "US",
                        "longitude": 40.71,
                        "latitude": -74.01
                    }
                }
            "#,
        )?;

        assert!(result.request.is_none());
        Ok(())
    }
}
