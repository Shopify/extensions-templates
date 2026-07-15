# Sidekick Discount App Action Link Extension

Sidekick app action link extensions let app developers expose actions to Sidekick in the Shopify Admin. This template registers a **discount** app action (a `shopify/Discount` intent) so that when a merchant asks Sidekick to create a discount your app powers, Sidekick navigates the merchant to the right page in your app to complete it.

Learn more about app action extensions in Shopify's [developer documentation](https://shopify.dev/docs/apps/build/sidekick/build-app-actions).

---

## Get started with this extension

This extension demonstrates registering a `create` intent for the `shopify/Discount` type on an `admin.app.intent.link` target. After deployment, Sidekick can invoke your app's action and navigate the merchant to your discount creation flow.

### Key files

- `shopify.extension.toml` - Extension configuration defining the intent target, URL, and action type
- `intent-schema.json` - Schema for the intent's input/output (references the `shopify/discount.json` baseline and returns the created discount's GID)
- `tools.json` - Tool definitions for actions Sidekick can perform within the intent context
- `instructions.md` - Guidelines for Sidekick on when and how to use your tools

### How it works

1. The extension registers an intent via `[[extensions.targeting.intents]]` with `type = "shopify/Discount"` and `action = "create"`.
2. `intent-schema.json` uses the `shopify-intent.json` meta-schema and references the `shopify/discount.json` baseline for the input. The `outputSchema` returns the created discount's GID.
3. When Sidekick invokes the action, the merchant is navigated to your app at the configured `url`.

### Customizing the intent

1. **Set your URL.** Replace the `YOUR_CREATE_ROUTE` placeholder in `url` in `shopify.extension.toml` with the page in your app that creates the discount. The golden path is a stable, unique route — either one create route (`/app/discount/new`) or a distinct route per discount type (`/app/discount/loyalty`). Avoid wildcard placeholders.
2. **Pin your function.** Replace `YOUR_DISCOUNT_FUNCTION_ID` in `intent-schema.json` with the ID of the discount function this extension creates discounts for. The `functionId` `matchValue` narrows a generic "create a discount" request to your function; without it, a generic create routes to the native discount picker instead. If your create route needs the function ID, you can also embed it in the URL (`/app/discount/{functionId}/new`).
3. Update `tools.json` with the tools Sidekick can use for your action.
4. Update `instructions.md` with guidelines for when and how Sidekick should use your tools.
5. Update the merchant-facing name and description in `locales/en.default.json`.

### Adding an `edit` intent

Editing is a separate extension (each `admin.app.intent.link` extension registers one intent). Use the dedicated [`discount_app_action_link_edit`](../discount-app-action-link-edit) template to generate it:

```
shopify app generate extension --template discount_app_action_link_edit
```

The edit template sets `action = "edit"`, uses a URL that carries the discount id (`/app/discount/:id`), and maps the discount GID to the `id` route param via a top-level `value` block instead of pinning a `functionId` (the edit route derives the function from the discount).

### Disambiguation (multiple functions)

If your app ships more than one discount function, register one intent per function (each in its own extension) and use the `functionId` `matchValue` to narrow each intent to a single function. Each registered intent counts toward the per-app intent limit below.

### Limits

- Maximum of 5 intents per app
- Maximum of 20 tools per app (shared across all extension types — data and action)
- Tool names: up to 64 characters
- Tool descriptions: up to 512 characters
- Description fields: 256-token limit
- Instructions file: 2,048-token limit

### Testing

Run `shopify app deploy` to deploy your extension and test it with Sidekick in the Shopify Admin.

> **Note:** `shopify app dev` may not upload the intent schema correctly. Use `shopify app deploy` for testing.
