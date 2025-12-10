use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function]
fn run(_input: schema::run::Input) -> Result<schema::FunctionRunResult> {
    let operations = vec![schema::Operation {
        add: schema::LocalPickupDeliveryOption {
            title: Some("Main St.".to_string()),
            cost: Some(Decimal(1.99)),
            pickup_location: schema::PickupLocation {
                location_handle: "2578303".to_string(),
                pickup_instruction: Some("Usually ready in 24 hours.".to_string()),
            },
            metafields: None,
        },
    }];

    // Build operations based on the input query response here.

    Ok(schema::FunctionRunResult { operations })
}
