use shopify_function::prelude::*;
use std::process;

pub mod generate_cart_run;
pub mod generate_delivery_run;

#[typegen("schema.graphql")]
pub mod schema {
    #[query("src/generate_cart_run.graphql")]
    pub mod generate_cart_run {}

    #[query("src/generate_delivery_run.graphql")]
    pub mod generate_delivery_run {}
}

fn main() {
    eprintln!("Please invoke a named export.");
    process::exit(1);
}
