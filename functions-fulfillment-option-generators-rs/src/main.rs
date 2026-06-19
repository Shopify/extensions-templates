use shopify_function::prelude::*;
use std::process;

pub mod fetch;
pub mod run;

#[typegen("schema.graphql")]
pub mod schema {
    // Type aliases must be defined for any custom scalars in the schema that the
    // typegen macro doesn't already know about. `Duration` is an ISO 8601 duration
    // string (e.g. "P2D").
    type Duration = String;

    #[query("src/run.graphql")]
    pub mod run {}

    #[query("src/fetch.graphql")]
    pub mod fetch {}
}

fn main() {
    log!("Please invoke a named export.");
    process::abort();
}
