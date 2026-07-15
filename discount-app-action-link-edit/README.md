# Sidekick Discount App Action Link Extension (Edit)

Sidekick app action link extensions let app developers expose actions to Sidekick in the Shopify Admin. This template registers an **edit** discount app action (an `edit` intent for the `shopify/Discount` type) so that when a merchant asks Sidekick to change a discount your app powers, Sidekick navigates the merchant to the right page in your app to edit it.

This is the edit counterpart to the [`discount_app_action_link`](../discount-app-action-link) (create) template. Each `admin.app.intent.link` extension registers exactly one intent, so create and edit are separate extensions with separate handles. Generate whichever intents your app supports.

Learn more about app action extensions in Shopify's [developer documentation](https://shopify.dev/docs/apps/build/sidekick/build-app-actions).

---

## Get started with this extension

This extension demonstrates registering an `edit` intent for the `shopify/Discount` type on an `admin.app.intent.link` target. After deployment, Sidekick can invoke your app's action and navigate the merchant to your discount edit flow for a specific discount.

### Key files

- `shopify.extension.toml` - Extension configuration defining the intent target, URL, and action type
- `intent-schema.json` - Schema for the intent's input/output (maps the discount GID to the `:id` route param and returns the edited discount's GID)

### How it works

1. The extension registers an intent via `[[extensions.targeting.intents]]` with `type = "shopify/Discount"` and `action = "edit"`.
2. `intent-schema.json` uses the `shopify-intent.json` meta-schema. A top-level `value` block maps the discount GID to the `id` route param (`fieldName: "id"`), so Sidekick fills the `:id` in the URL. The `outputSchema` returns the edited discount's GID.
3. When Sidekick invokes the action, the merchant is navigated to your app at the configured `url` with the discount id filled in.

### Why no `functionId`

Unlike the create template, the edit route derives the function from the existing discount, so there's no `functionId` `matchValue` to pin. The intent only needs the discount id. That also means the edit intent is navigation-only — it doesn't ship a `tools.json` or `instructions.md`.

### Customizing the intent

1. **Set your URL.** Point `url` in `shopify.extension.toml` at the page in your app that edits a discount. Customize the path prefix, but keep the `:id` param so the discount id is passed through (for example `/app/discount/:id`).
2. Update the merchant-facing name and description in `locales/en.default.json`.

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
