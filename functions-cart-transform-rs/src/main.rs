use shopify_function::prelude::*;
use std::process;

pub mod cart_transform_run;

#[typegen("schema.graphql")]
pub mod schema {
    #[query("src/cart_transform_run.graphql")]
    pub mod cart_transform_run {}
}

fn main() {
    eprintln!("Please invoke a named export.");
    process::exit(1);
}
