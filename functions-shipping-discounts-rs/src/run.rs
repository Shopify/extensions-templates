use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[derive(Deserialize, Default)]
#[shopify_function(rename_all = "camelCase")]
pub struct Configuration {}

#[shopify_function]
fn run(input: schema::run::Input) -> Result<schema::FunctionRunResult> {
    let no_discount = schema::FunctionRunResult { discounts: vec![] };

    let _config = match input.discount_node().metafield() {
        Some(metafield) => metafield.json_value(),
        None => return Ok(no_discount),
    };

    Ok(schema::FunctionRunResult { discounts: vec![] })
}
