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

    // Discount only the groups with shipping options.
    let targets: Vec<schema::DeliveryDiscountCandidateTarget> = input
        .cart()
        .delivery_groups()
        .iter()
        // If you also want to apply a discount for other delivery methods (e.g. LOCAL), ensure to account for that here.
        .filter(|group| {
            group.delivery_options().iter().any(|option| {
                matches!(
                    option.delivery_method_type(),
                    schema::DeliveryMethod::Shipping
                )
            })
        })
        .map(|group| {
            schema::DeliveryDiscountCandidateTarget::DeliveryGroup(schema::DeliveryGroupTarget {
                id: group.id().clone(),
            })
        })
        .collect();

    if targets.is_empty() {
        return Ok(schema::CartDeliveryOptionsDiscountsGenerateRunResult { operations: vec![] });
    }

    Ok(schema::CartDeliveryOptionsDiscountsGenerateRunResult {
        operations: vec![schema::DeliveryOperation::DeliveryDiscountsAdd(
            schema::DeliveryDiscountsAddOperation {
                selection_strategy: schema::DeliveryDiscountSelectionStrategy::All,
                candidates: vec![schema::DeliveryDiscountCandidate {
                    targets,
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
