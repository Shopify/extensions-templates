use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function]
fn cart_validations_generate_run(
    input: schema::cart_validations_generate_run::Input,
) -> Result<schema::CartValidationsGenerateRunResult> {
    let mut operations = Vec::new();
    let mut errors = Vec::new();

    if input
        .cart()
        .lines()
        .iter()
        .map(|line| *line.quantity())
        .any(|quantity| quantity > 1)
    {
        errors.push(schema::ValidationError {
            message: "Not possible to order more than one of each".to_owned(),
            target: "$.cart".to_owned(),
        })
    }
    let operation = schema::ValidationAddOperation { errors };
    operations.push(schema::Operation::ValidationAdd(operation));

    Ok(schema::CartValidationsGenerateRunResult { operations })
}
