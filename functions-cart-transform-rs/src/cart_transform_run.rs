use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function]
fn cart_transform_run(
    _input: schema::cart_transform_run::CartTransformRunInput,
) -> Result<schema::CartTransformRunResult> {
    let _ = _input;
    let no_changes = schema::CartTransformRunResult { operations: vec![] };

    Ok(no_changes)
}
