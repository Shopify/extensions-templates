use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

use serde_json::Value;

type TimeWithoutTimezone = String;

#[shopify_function]
fn run(input: schema::run::Input) -> Result<schema::CartFulfillmentOptionsGenerateRunResult> {
    // The HTTP response from the `fetch` target is available here in `fetch_result`.
    let Some(fetch_result) = input.fetch_result() else {
        return Ok(empty_result());
    };

    if *fetch_result.status() != 200 {
        return Ok(empty_result());
    }

    let Some(body) = fetch_result.body() else {
        return Ok(empty_result());
    };

    let Ok(external_api_data) = serde_json::from_str::<Value>(body) else {
        return Ok(empty_result());
    };

    let Some(external_api_delivery_points) = external_api_data["deliveryPoints"].as_array() else {
        return Ok(empty_result());
    };

    // Providers and service points are deduplicated into the `references` block
    // and referenced from operations by their handle, to keep the payload small.
    let provider = build_provider();
    let service_points: Vec<schema::ServicePoint> = external_api_delivery_points
        .iter()
        .filter_map(build_service_point)
        .collect();

    // Generate one fulfillment option per service point, for every input fulfillment.
    let mut operations = Vec::new();
    for fulfillment in input.fulfillments().iter() {
        for service_point in service_points.iter() {
            operations.push(schema::Operation {
                fulfillment_options_add: schema::FulfillmentOptionsAddOperation {
                    fulfillment_handle: fulfillment.handle().to_string(),
                    code: None,
                    title: Some(service_point.name.clone()),
                    instructions: None,
                    provider_handle: Some(provider.handle.clone()),
                    destination_service_point_handle: Some(service_point.handle.clone()),
                    cost: None,
                    metafields: Some(vec![]),
                },
            });
        }
    }

    Ok(schema::CartFulfillmentOptionsGenerateRunResult {
        references: Some(schema::References {
            providers: Some(vec![provider]),
            service_points: Some(service_points),
        }),
        operations,
    })
}

fn empty_result() -> schema::CartFulfillmentOptionsGenerateRunResult {
    schema::CartFulfillmentOptionsGenerateRunResult {
        operations: vec![],
        references: None,
    }
}

fn build_provider() -> schema::Provider {
    schema::Provider {
        handle: "shopify-demo-provider".to_string(),
        external_id: "shopify-demo".to_string(),
        name: "Shopify Functions Demo".to_string(),
        logo_url: "https://cdn.shopify.com/s/files/1/0628/3830/9033/files/shopify_icon_146101.png?v=1706120545".to_string(),
    }
}

fn build_service_point(external_api_delivery_point: &Value) -> Option<schema::ServicePoint> {
    let location = &external_api_delivery_point["location"];
    let address_components = &location["addressComponents"];
    let geometry = &location["geometry"]["location"];
    let point_id = external_api_delivery_point["pointId"].as_str()?;

    Some(schema::ServicePoint {
        handle: format!("service-point-{point_id}"),
        external_id: point_id.to_string(),
        name: external_api_delivery_point["pointName"]
            .as_str()?
            .to_string(),
        address_1: format!(
            "{} {}",
            address_components["streetNumber"].as_str()?,
            address_components["route"].as_str()?
        ),
        address_2: None,
        city: address_components["locality"].as_str()?.to_string(),
        country_code: address_components["countryCode"].as_str()?.to_string(),
        province_code: address_components["administrativeArea"]["code"]
            .as_str()
            .map(|code| code.to_string()),
        zip: address_components["postalCode"]
            .as_str()
            .map(|zip| zip.to_string()),
        phone: None,
        latitude: geometry["lat"].as_f64().unwrap_or_default(),
        longitude: geometry["lng"].as_f64().unwrap_or_default(),
        business_hours: build_business_hours(external_api_delivery_point),
    })
}

/// Transforms the opening hours of a delivery point into a vector of `BusinessHours` objects.
/// Each day's opening hours are represented using a `BusinessHours` object as follows:
/// "Monday: 9:00 AM – 5:00 PM" is transformed to {day: Monday, periods: [{opening_time: "09:00:00", closing_time: "17:00:00"}]}
/// "Tuesday: Closed" is transformed to {day: Tuesday, periods: []}
fn build_business_hours(external_api_delivery_point: &Value) -> Option<Vec<schema::BusinessHours>> {
    if external_api_delivery_point["openingHours"].is_null() {
        return None;
    }

    let business_hours = external_api_delivery_point["openingHours"]["weekdayText"]
        .as_array()
        .unwrap()
        .iter()
        .map(|day| {
            let day_parts: Vec<&str> = day.as_str().unwrap().split(": ").collect();
            let day_name = schema::Weekday::from_str(&day_parts[0].to_uppercase());
            if day_parts[1] == "Closed" {
                schema::BusinessHours {
                    day: day_name,
                    periods: vec![],
                }
            } else {
                let opening_closing_times: Vec<&str> = day_parts[1].split(" – ").collect();
                schema::BusinessHours {
                    day: day_name,
                    periods: vec![schema::BusinessHoursPeriod {
                        opening_time: format_time(opening_closing_times[0]),
                        closing_time: format_time(opening_closing_times[1]),
                    }],
                }
            }
        })
        .collect();

    Some(business_hours)
}

/// Converts a time string from 12-hour to 24-hour format.
/// Example: "9:00 AM" => "09:00:00", "5:00 PM" => "17:00:00"
fn format_time(time: &str) -> TimeWithoutTimezone {
    let time_parts: Vec<&str> = time.split_whitespace().collect();
    let hour_min: Vec<&str> = time_parts[0].split(':').collect();
    let hour: u32 = hour_min[0].parse().unwrap();
    let min: &str = hour_min[1];
    let period: &str = time_parts[1];

    let hour_in_24_format = match period {
        "AM" => {
            if hour == 12 {
                0
            } else {
                hour
            }
        }
        "PM" => {
            if hour == 12 {
                hour
            } else {
                hour + 12
            }
        }
        _ => hour,
    };

    format!("{hour_in_24_format:02}:{min:02}:00")
}
