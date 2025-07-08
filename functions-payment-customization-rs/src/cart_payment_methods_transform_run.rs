use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[derive(Deserialize, Default, PartialEq)]
#[shopify_function(rename_all = "camelCase")]
pub struct Configuration {}

#[shopify_function]
fn cart_payment_methods_transform_run(
    input: schema::cart_payment_methods_transform_run::Input,
) -> Result<schema::CartPaymentMethodsTransformRunResult> {
    let no_changes = schema::CartPaymentMethodsTransformRunResult { operations: vec![] };

    let _config = match input.payment_customization().metafield() {
        Some(metafield) => metafield.json_value(),
        None => return Ok(no_changes),
    };

    Ok(schema::CartPaymentMethodsTransformRunResult { operations: vec![] })
}

#[cfg(test)]
mod tests {
    use super::*;
    use shopify_function::{run_function_with_input, Result};

    #[test]
    fn test_result_contains_no_operations() -> Result<()> {
        let result = run_function_with_input(
            cart_payment_methods_transform_run,
            r#"
                {
                    "paymentCustomization": {
                        "metafield": null
                    }
                }
            "#,
        )?;
        let expected = schema::CartPaymentMethodsTransformRunResult { operations: vec![] };

        assert_eq!(result, expected);
        Ok(())
    }
}
