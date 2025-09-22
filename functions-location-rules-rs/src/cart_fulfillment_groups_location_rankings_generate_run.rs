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
