# extensions-templates

This repo contains the templates used by `@shopify/app` CLI when generating extensions.

## Create an extension template

Follow this guide to [create an extension template](https://vault.shopify.io/page/Create-an-extension-template~cqQS.md)

## Local Functions Development

The following instructions are for building and testing Function templates, which follow the pattern `functions-*-[rs|js]`.

### Expand Liquid Templates

If you wish to build and test the `functions` templates, you must first expand any `.liquid` templates with the following command.

```shell
yarn
yarn expand-liquid [rust|vanilla-js|typescript] <project-name-without-suffix>
# example: yarn expand-liquid rust functions-cart-checkout-validation

# optionally specify only the language argument to expand all functions projects for that language
yarn expand-liquid rust
yarn expand-liquid typescript
yarn expand-liquid vanilla-js
yarn expand-liquid wasm
```

### JavaScript / TypeScript

```shell
# Setup node workspaces
yarn
# Generate types from GraphQL schemas
yarn js-typegen
# Run tests
yarn js-test
```

### Rust

```shell
# Lint
cargo fmt
cargo clippy -- -D warnings
# Run tests
cargo test
# Build .wasm packages
cargo build --release --target wasm32-wasip1
```
