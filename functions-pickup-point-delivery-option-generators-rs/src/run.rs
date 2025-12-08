use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

use serde_json::Value;

type TimeWithoutTimezone = String;

#[shopify_function]
fn run(input: schema::run::Input) -> Result<schema::FunctionRunResult> {
    match input.fetch_result() {
        Some(fetch_result) => {
            // Check if we have a valid body and it parses as JSON
            if let Some(body) = fetch_result.body() {
                if let Ok(external_api_data) = serde_json::from_str::<Value>(body) {
                    // Check if we have a deliveryPoints array
                    if let Some(external_api_delivery_points) =
                        external_api_data["deliveryPoints"].as_array()
                    {
                        let operations = build_pickup_point_delivery_option_operations(
                            external_api_delivery_points,
                        );
                        return Ok(schema::FunctionRunResult { operations });
                    }
                }
            }
            Ok(schema::FunctionRunResult { operations: vec![] })
        }
        _ => Ok(schema::FunctionRunResult { operations: vec![] }),
    }
}

fn build_pickup_point_delivery_option_operations(
    external_api_delivery_points: &[Value],
) -> Vec<schema::Operation> {
    external_api_delivery_points
        .iter()
        .filter_map(|external_api_delivery_point| {
            build_pickup_point_delivery_option(external_api_delivery_point)
        })
        .map(|op| schema::Operation { add: op })
        .collect()
}

fn build_pickup_point_delivery_option(
    external_api_delivery_point: &Value,
) -> Option<schema::PickupPointDeliveryOption> {
    Some(schema::PickupPointDeliveryOption {
        cost: None,
        pickup_point: schema::PickupPoint {
            external_id: external_api_delivery_point["pointId"]
                .as_str()
                .unwrap()
                .to_string(),
            name: external_api_delivery_point["pointName"]
                .as_str()
                .unwrap()
                .to_string(),
            provider: build_provider(),
            address: build_address(external_api_delivery_point)?,
            business_hours: build_business_hours(external_api_delivery_point),
        },
        metafields: Some(vec![]),
    })
}

fn build_provider() -> schema::Provider {
    schema::Provider {
        name: "Shopify Rust Demo".to_string(),
        logo_url: "https://cdn.shopify.com/s/files/1/0628/3830/9033/files/shopify_icon_146101.png?v=1706120545".to_string(),
    }
}

fn build_address(external_api_delivery_point: &Value) -> Option<schema::PickupAddress> {
    let address = schema::PickupAddress {
        address_1: format!(
            "{} {}",
            external_api_delivery_point["location"]["addressComponents"]["streetNumber"]
                .as_str()
                .unwrap(),
            external_api_delivery_point["location"]["addressComponents"]["route"]
                .as_str()
                .unwrap()
        ),
        address_2: None,
        city: external_api_delivery_point["location"]["addressComponents"]["locality"]
            .as_str()
            .unwrap()
            .to_string(),
        country: Some(
            external_api_delivery_point["location"]["addressComponents"]["country"]
                .as_str()
                .unwrap()
                .to_string(),
        ),
        country_code: external_api_delivery_point["location"]["addressComponents"]["countryCode"]
            .as_str()
            .unwrap()
            .to_string(),
        latitude: external_api_delivery_point["location"]["geometry"]["location"]["lat"]
            .as_f64()
            .unwrap_or_default(),
        longitude: external_api_delivery_point["location"]["geometry"]["location"]["lng"]
            .as_f64()
            .unwrap_or_default(),
        phone: None,
        province: Some(
            external_api_delivery_point["location"]["addressComponents"]["administrativeArea"]
                ["name"]
                .as_str()
                .unwrap()
                .to_string(),
        ),
        province_code: Some(
            external_api_delivery_point["location"]["addressComponents"]["administrativeArea"]
                ["code"]
                .as_str()
                .unwrap()
                .to_string(),
        ),
        zip: Some(
            external_api_delivery_point["location"]["addressComponents"]["postalCode"]
                .as_str()
                .unwrap()
                .to_string(),
        ),
    };
    Some(address)
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

