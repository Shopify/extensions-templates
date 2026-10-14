# extensions-templates

This repo contains the templates used by `@shopify/app` CLI when generating extensions.

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
cargo build --release --target wasm32-unknown-unknown
```

### Refreshing `schema.graphql`

After bumping `api_version` in the `shopify.extension.toml.liquid` files (or when a schema changes upstream), use `refresh-function-schemas.sh` to regenerate the committed `schema.graphql` for every function template. The script expands liquid templates locally, symlinks each flavor into a linked Shopify app's `extensions/` directory, and runs `shopify app function schema` against each template's own `api_version`. Nothing is cloned from GitHub — whatever is in your working tree is what gets refreshed.

Prereqs: `bash`, `git`, `jq`, `yarn`, `@shopify/cli`, and a dev app you've already linked via `shopify app config link`.

```shell
# refresh every function's schema from the local working tree
./refresh-function-schemas.sh --app-dir ~/path/to/linked-app

# refresh only a subset
./refresh-function-schemas.sh --app-dir ~/path/to/linked-app --only cart_transform,discount

# leave the generated `schemagen-*` symlinks in the app for inspection
./refresh-function-schemas.sh --app-dir ~/path/to/linked-app --keep
```

Run `./refresh-function-schemas.sh --help` for the full flag list. After it finishes, review with `git status` / `git diff` and commit any schema changes.
