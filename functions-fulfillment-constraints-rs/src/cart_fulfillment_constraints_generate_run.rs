use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function]
fn cart_fulfillment_constraints_generate_run(
    _input: schema::cart_fulfillment_constraints_generate_run::CartFulfillmentConstraintsGenerateRunInput,
) -> Result<schema::CartFulfillmentConstraintsGenerateRunResult> {
    let operations = vec![];

    // Build operations based on the input query response here.

    Ok(schema::CartFulfillmentConstraintsGenerateRunResult { operations })
}
