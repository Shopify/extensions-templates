use shopify_function::prelude::*;
use std::process;

pub mod cart_delivery_options_transform_run;

#[typegen("schema.graphql")]
pub mod schema {
    #[query("src/cart_delivery_options_transform_run.graphql", custom_scalar_overrides = {
        "CartDeliveryOptionsTransformRunInput.deliveryCustomization.metafield.jsonValue" => super::cart_delivery_options_transform_run::Configuration,
    })]
    pub mod cart_delivery_options_transform_run {}
}

fn main() {
    log!("Please invoke a named export.");
    process::abort();
}
