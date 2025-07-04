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

#[cfg(test)]
mod tests {
    use super::*;
    use shopify_function::{run_function_with_input, Result};

    #[test]
    fn test_result_contains_no_operations() -> Result<()> {
        let result = run_function_with_input(
            cart_fulfillment_constraints_generate_run,
            r#"
              {
                "cart": {
                  "deliverableLines": [
                    {
                      "id": "gid://shopify/DeliverableCartLine/1"
                    },
                    {
                      "id": "gid://shopify/DeliverableCartLine/2"
                    }
                  ]
                },
                "fulfillmentConstraintRule": {
                  "metafield": null
                }
              }
            "#,
        )?;

        let expected = schema::CartFulfillmentConstraintsGenerateRunResult { operations: vec![] };

        assert_eq!(result, expected);
        Ok(())
    }
}
