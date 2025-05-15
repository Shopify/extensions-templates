use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[derive(Deserialize, Default)]
#[shopify_function(rename_all= "camelCase" )]
pub struct Configuration {}

#[shopify_function]
fn run(input: schema::run::Input) -> Result<schema::FunctionRunResult> {
    let no_discount = schema::FunctionRunResult {
        discounts: vec![],
        discount_application_strategy: schema::DiscountApplicationStrategy::First,
    };

    let _config = match input.discount_node().metafield() {
        Some(metafield) => metafield.json_value(),
        None => return Ok(no_discount),
    };

    Ok(schema::FunctionRunResult {
        discounts: vec![],
        discount_application_strategy: schema::DiscountApplicationStrategy::First,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use shopify_function::{run_function_with_input, Result};

    #[test]
    fn test_result_contains_no_discounts() -> Result<()> {
        let result = run_function_with_input(
            run,
            r#"
                {
                    "discountNode": {
                        "metafield": null
                    }
                }
            "#,
        )?;
        let expected = schema::FunctionRunResult {
            discounts: vec![],
            discount_application_strategy: schema::DiscountApplicationStrategy::First,
        };

        assert_eq!(result, expected);
        Ok(())
    }
}
