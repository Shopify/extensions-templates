use shopify_function::prelude::*;
use std::process;

pub mod cart_validations_generate_run;

#[typegen("schema.graphql")]
pub mod schema {
    #[query("src/cart_validations_generate_run.graphql")]
    pub mod run {}
}

fn main() {
    eprintln!("Please invoke a named export.");
    process::exit(1);
}
