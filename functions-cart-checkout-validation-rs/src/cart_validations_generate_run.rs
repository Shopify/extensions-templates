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

#[cfg(test)]
mod tests {
    use super::*;
    use shopify_function::{run_function_with_input, Result};

    #[test]
    fn test_result_contains_single_error_when_quantity_exceeds_one() -> Result<()> {
        let result = run_function_with_input(
            cart_validations_generate_run,
            r#"
                {
                    "cart": {
                        "lines": [
                            {
                                "quantity": 3
                            }
                        ]
                    }
                }
            "#,
        )?;
        let expected = schema::CartValidationsGenerateRunResult {
            operations: vec![schema::Operation::ValidationAdd(
                schema::ValidationAddOperation {
                    errors: vec![schema::ValidationError {
                        message: "Not possible to order more than one of each".to_owned(),
                        target: "$.cart".to_owned(),
                    }],
                },
            )],
        };

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_result_contains_no_errors_when_quantity_is_one() -> Result<()> {
        let result = run_function_with_input(
            cart_validations_generate_run,
            r#"
                {
                    "cart": {
                        "lines": [
                            {
                                "quantity": 1
                            }
                        ]
                    }
                }
            "#,
        )?;
        let expected = schema::CartValidationsGenerateRunResult {
            operations: vec![schema::Operation::ValidationAdd(
                schema::ValidationAddOperation { errors: vec![] },
            )],
        };

        assert_eq!(result, expected);
        Ok(())
    }
}
