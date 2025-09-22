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
