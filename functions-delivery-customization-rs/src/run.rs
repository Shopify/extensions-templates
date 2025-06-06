use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[derive(Deserialize, Default)]
#[shopify_function(rename_all = "camelCase")]
pub struct Configuration {}

#[shopify_function]
fn run(input: schema::run::Input) -> Result<schema::FunctionRunResult> {
    let no_changes = schema::FunctionRunResult { operations: vec![] };

    let _config = match input.delivery_customization().metafield() {
        Some(metafield) => metafield.json_value(),
        None => return Ok(no_changes),
    };

    Ok(schema::FunctionRunResult { operations: vec![] })
}

#[cfg(test)]
mod tests {
    use super::*;
    use shopify_function::{run_function_with_input, Result};

    #[test]
    fn test_result_contains_no_operations() -> Result<()> {
        use schema::FunctionRunResult;

        let result = run_function_with_input(
            run,
            r#"
                {
                    "deliveryCustomization": {
                        "metafield": null
                    }
                }
            "#,
        )?;
        let expected = FunctionRunResult { operations: vec![] };

        assert_eq!(result, expected);
        Ok(())
    }
}
