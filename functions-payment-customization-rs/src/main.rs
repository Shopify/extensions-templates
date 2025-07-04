use shopify_function::prelude::*;
use std::process;

pub mod cart_payment_methods_transform_run;

#[typegen("schema.graphql")]
pub mod schema {
    #[query("src/cart_payment_methods_transform_run.graphql", custom_scalar_overrides = {
        "Input.paymentCustomization.metafield.jsonValue" => super::cart_payment_methods_transform_run::Configuration,
    })]
    pub mod run {}
}

fn main() {
    eprintln!("Please invoke a named export.");
    process::exit(1);
}
