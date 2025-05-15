use shopify_function::prelude::*;
use std::process;

pub mod run;

#[typegen("schema.graphql")]
pub mod schema {
    #[query("src/run.graphql", custom_scalar_overrides = {
        "Input.discountNode.metafield.jsonValue" => super::run::Configuration,
    })]
    pub mod run {}
}

fn main() {
    eprintln!("Please invoke a named export.");
    process::exit(1);
}
