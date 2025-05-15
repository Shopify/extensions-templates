use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function]
fn run(_input: schema::run::Input) -> Result<schema::FunctionRunResult> {
    let no_discounts = schema::FunctionRunResult {
        line_discounts: Some(vec![]),
        displayable_errors: Some(vec![]),
    };

    Ok(no_discounts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use shopify_function::run_function_with_input;

    #[test]
    fn test_result_with_no_discounts() -> Result<()> {
        let result = run_function_with_input(
            run,
            r#"
            {
              "shop": {
                "metafield": null
              }
            }
            "#,
        )?;
        let expected = schema::FunctionRunResult {
            line_discounts: Some(vec![]),
            displayable_errors: Some(vec![]),
        };

        assert_eq!(result, expected);
        Ok(())
    }
}
