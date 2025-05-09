use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function]
fn run(_input: schema::run::Input) -> Result<schema::FunctionRunResult> {
    let no_changes = schema::FunctionRunResult { operations: vec![] };

    Ok(no_changes)
}
