# Sidekick Discount App Action Link Extension

Sidekick app action link extensions let app developers expose actions to Sidekick in the Shopify Admin. This template registers **two discount app actions** in one extension module — a `create` intent and an `edit` intent for the `shopify/Discount` type — so that when a merchant asks Sidekick to create or change a discount your app powers, Sidekick navigates the merchant to the right page in your app to complete it.

Apps that power discounts with a function typically want both intents so Sidekick can drive the full create + edit flow. If your app only supports one, delete the other `[[extensions]]` block in `shopify.extension.toml`.

Learn more about app action extensions in Shopify's [developer documentation](https://shopify.dev/docs/apps/build/sidekick/build-app-actions).

---

## Get started with this extension

The module defines two `admin.app.intent.link` extensions, each registering one intent:

- **Create** (`action = "create"`) — opens your app's discount creation flow. Pinned to a single discount function via the `functionId` `matchValue` in `intent-schema.json`.
- **Edit** (`action = "edit"`) — opens your app's discount edit flow for a specific discount. Maps the discount GID to the `:id` route param.

Both intents are navigation-only: Sidekick routes the merchant to your app to complete the action, and the intents alone make the extension discoverable by Sidekick (no `tools.json` needed). The create intent ships an `instructions.md` telling Sidekick to pass your `functionId` so it resolves directly to your app's create flow.

Each `admin.app.intent.link` extension registers exactly one intent, which is why create and edit are two `[[extensions]]` blocks (with distinct handles `{handle}-create` and `{handle}-edit`) rather than one.

### Key files

- `shopify.extension.toml` - Two `[[extensions]]` blocks defining the create and edit intent targets, URLs, and action types
- `intent-schema.json` - Create intent schema (references the `shopify/discount.json` baseline, pins `functionId`, returns the created discount's GID)
- `intent-schema-edit.json` - Edit intent schema (maps the discount GID to the `:id` route param and returns the edited discount's GID)
- `instructions.md` - Tells Sidekick to pass your `functionId` when invoking `create` (create intent only)

### How it works

1. Each extension registers an intent via `[[extensions.targeting.intents]]` with `type = "shopify/Discount"` and `action = "create"` or `"edit"`.
2. The create schema uses the `shopify-intent.json` meta-schema and references the `shopify/discount.json` baseline for the input; `functionId` `matchValue` narrows a generic "create a discount" request to your function. The edit schema uses a top-level `value` block to map the discount GID to the `id` route param (`fieldName: "id"`), so Sidekick fills the `:id` in the URL.
3. When Sidekick invokes an action, the merchant is navigated to your app at the configured `url` (with the discount id filled in for edit).

### Customizing the intents

1. **Set your URLs.** Replace `YOUR_CREATE_ROUTE` and `YOUR_EDIT_ROUTE` in `shopify.extension.toml` with the routes in your app that create and edit the discount. The whole path is up to your app (there's no required prefix), but each must be a stable, unique route (for example `/discounts/loyalty/new` and `/discounts/:id`). Keep the `:id` param on the edit route. Avoid wildcard placeholders.
2. **Pin your function.** Replace `YOUR_DISCOUNT_FUNCTION_ID` in `intent-schema.json` and `instructions.md` with the ID of the discount function this extension creates discounts for. The `functionId` `matchValue` narrows a generic "create a discount" request to your function; without it, a generic create routes to the native discount picker instead. If your create route needs the function ID, you can also embed it in the URL (`/discounts/{functionId}/new`). The edit route derives the function from the existing discount, so it needs no `functionId`.
3. Update the merchant-facing names and descriptions in `locales/en.default.json` (under the `create` and `edit` keys).

### Get your function ID

`YOUR_DISCOUNT_FUNCTION_ID` (in `intent-schema.json` and `instructions.md`) is the ID of the discount function this extension targets. Two ways to find it:

- **CLI** — after `shopify app deploy` (or `shopify app function deploy`), the CLI prints the registered function's ID.
- **Admin API** — query [`shopifyFunctions`](https://shopify.dev/docs/api/admin-graphql/latest/queries/shopifyFunctions):
  ```graphql
  {
    shopifyFunctions(first: 50) {
      nodes {
        id
        title
        description
      }
    }
  }
  ```

### Disambiguation (multiple functions)

If your app ships more than one discount function, register one create intent per function (each in its own extension) and use the `functionId` `matchValue` to narrow each intent to a single function. Each registered intent counts toward the per-app intent limit below.

### Limits

- Maximum of 5 intents per app
- Description fields: 256-token limit

### Testing

Run `shopify app deploy` to deploy your extensions and test them with Sidekick in the Shopify Admin.

> **Note:** `shopify app dev` may not upload the intent schema correctly. Use `shopify app deploy` for testing.
