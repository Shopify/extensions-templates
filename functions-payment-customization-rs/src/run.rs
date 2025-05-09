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
    let no_changes = schema::FunctionRunResult { operations: vec![] };

    let _config = match input.payment_customization().metafield() {
        Some(metafield) => Configuration::from_str(metafield.value()),
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
        let result = run_function_with_input(
            run,
            r#"
                {
                    "paymentCustomization": {
                        "metafield": null
                    }
                }
            "#,
        )?;
        let expected = schema::FunctionRunResult { operations: vec![] };

        assert_eq!(result, expected);
        Ok(())
    }
}
