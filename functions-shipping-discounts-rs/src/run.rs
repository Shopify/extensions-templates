use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Configuration {}

impl Configuration {
    fn from_str(value: &str) -> Self {
        serde_json::from_str(value).expect("Unable to parse configuration value from metafield")
    }
}

#[shopify_function]
fn run(input: schema::run::Input) -> Result<schema::FunctionRunResult> {
    let no_discount = schema::FunctionRunResult { discounts: vec![] };

    let _config = match input.discount_node().metafield() {
        Some(metafield) => Configuration::from_str(metafield.value()),
        None => return Ok(no_discount),
    };

    Ok(schema::FunctionRunResult { discounts: vec![] })
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
        let expected = schema::FunctionRunResult { discounts: vec![] };

        assert_eq!(result, expected);
        Ok(())
    }
}
