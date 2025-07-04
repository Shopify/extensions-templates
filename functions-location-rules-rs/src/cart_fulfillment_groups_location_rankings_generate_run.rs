use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function]
fn cart_fulfillment_groups_location_rankings_generate_run(
    input: schema::cart_fulfillment_groups_location_rankings_generate_run::CartFulfillmentGroupsLocationRankingsGenerateRunInput,
) -> Result<schema::CartFulfillmentGroupsLocationRankingsGenerateRunResult> {
    let operations = input
        .fulfillment_groups()
        .iter()
        .map(|group| {
            let rankings = group
                .inventory_location_handles()
                .iter()
                .map(|location_handle| schema::RankedLocation {
                    location_handle: location_handle.clone(),
                    rank: 0,
                })
                .collect::<Vec<schema::RankedLocation>>();

            schema::Operation::FulfillmentGroupLocationRankingAdd(
                schema::FulfillmentGroupLocationRankingAddOperation {
                    fulfillment_group_handle: group.handle().clone(),
                    rankings,
                },
            )
        })
        .collect();

    Ok(schema::CartFulfillmentGroupsLocationRankingsGenerateRunResult { operations })
}

#[cfg(test)]
mod tests {
    use super::schema;
    use crate::cart_fulfillment_groups_location_rankings_generate_run::cart_fulfillment_groups_location_rankings_generate_run;
    use shopify_function::{run_function_with_input, Result};

    #[test]
    fn test_result_ranks_all_locations_zero() -> Result<()> {
        let result = run_function_with_input(
            cart_fulfillment_groups_location_rankings_generate_run,
            r#"
                {
                    "fulfillmentGroups": [{
                        "handle": "123",
                        "inventoryLocationHandles": ["456"]
                    }]
                }
            "#,
        )?;
        let expected = schema::CartFulfillmentGroupsLocationRankingsGenerateRunResult {
            operations: vec![schema::Operation::FulfillmentGroupLocationRankingAdd(
                schema::FulfillmentGroupLocationRankingAddOperation {
                    fulfillment_group_handle: "123".to_string(),
                    rankings: vec![schema::RankedLocation {
                        location_handle: "456".to_string(),
                        rank: 0,
                    }],
                },
            )],
        };

        assert_eq!(result, expected);
        Ok(())
    }
}
