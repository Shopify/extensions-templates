use shopify_function::prelude::*;
use std::process;

pub mod fetch;
pub mod run;
#[typegen("schema.graphql")]
pub mod schema {
    #[query("src/run.graphql")]
    pub mod run {}

    #[query("src/fetch.graphql")]
    pub mod fetch {}
}

fn main() {
    log!("Please invoke a named export.");
    process::abort();
}
