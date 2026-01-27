use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function]
fn cart_delivery_options_discounts_generate_run(
    input: schema::cart_delivery_options_discounts_generate_run::Input,
) -> Result<schema::CartDeliveryOptionsDiscountsGenerateRunResult> {
    let has_shipping_discount_class = input
        .discount()
        .discount_classes()
        .contains(&schema::DiscountClass::Shipping);

    if !has_shipping_discount_class {
        return Ok(schema::CartDeliveryOptionsDiscountsGenerateRunResult { operations: vec![] });
    }

    let first_delivery_group = input
        .cart()
        .delivery_groups()
        .first()
        .ok_or("No delivery groups found")?;

    Ok(schema::CartDeliveryOptionsDiscountsGenerateRunResult {
        operations: vec![schema::DeliveryOperation::DeliveryDiscountsAdd(
            schema::DeliveryDiscountsAddOperation {
                selection_strategy: schema::DeliveryDiscountSelectionStrategy::All,
                candidates: vec![schema::DeliveryDiscountCandidate {
                    targets: vec![schema::DeliveryDiscountCandidateTarget::DeliveryGroup(
                        schema::DeliveryGroupTarget {
                            id: first_delivery_group.id().clone(),
                        },
                    )],
                    value: schema::DeliveryDiscountCandidateValue::Percentage(schema::Percentage {
                        value: Decimal(100.0),
                    }),
                    message: Some("FREE DELIVERY".to_string()),
                    associated_discount_code: None,
                }],
            },
        )],
    })
}
