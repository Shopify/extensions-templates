use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[derive(Deserialize, Default)]
#[shopify_function(rename_all = "camelCase")]
pub struct Configuration {}

#[shopify_function]
fn cart_delivery_options_transform_run(
    input: schema::cart_delivery_options_transform_run::CartDeliveryOptionsTransformRunInput,
) -> Result<schema::CartDeliveryOptionsTransformRunResult> {
    let no_changes = schema::CartDeliveryOptionsTransformRunResult { operations: vec![] };

    let _config = match input.delivery_customization().metafield() {
        Some(metafield) => metafield.json_value(),
        None => return Ok(no_changes),
    };

    Ok(schema::CartDeliveryOptionsTransformRunResult { operations: vec![] })
}

#[cfg(test)]
mod tests {
    use super::*;
    use shopify_function::{run_function_with_input, Result};

    #[test]
    fn test_result_contains_no_operations() -> Result<()> {
        use schema::CartDeliveryOptionsTransformRunResult;

        let result = run_function_with_input(
            cart_delivery_options_transform_run,
            r#"
                {
                    "deliveryCustomization": {
                        "metafield": null
                    }
                }
            "#,
        )?;
        let expected = CartDeliveryOptionsTransformRunResult { operations: vec![] };

        assert_eq!(result, expected);
        Ok(())
    }
}
