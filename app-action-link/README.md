# Sidekick App Action Link Extension

Sidekick app action link extensions enable app developers to expose actions to Sidekick in the Shopify Admin. When a merchant asks Sidekick to perform an action, Sidekick can navigate the merchant to the right page in your app to complete it.

Learn more about app action extensions in Shopify's [developer documentation](https://shopify.dev/docs/apps/build/sidekick/build-app-actions).

---

## Get started with this extension

This extension demonstrates registering an app action (intent) for Sidekick. After deployment, Sidekick will be able to invoke your app's action and navigate the merchant to the correct page.

### Key files

- `shopify.extension.toml` - Extension configuration defining the intent target and action type
- `intent-schema.json` - Schema definition for the intent's input parameters
- `tools.json` - Tool definitions for actions Sidekick can perform within the intent context
- `instructions.md` - Guidelines for Sidekick on when and how to use your tools

### How it works

1. The extension registers an intent via the `[[extensions.targeting.intents]]` config
2. The `type` field (e.g., `application/email`) tells Sidekick what kind of data the action works with
3. The `action` field (e.g., `open`) tells Sidekick what the action does
4. When Sidekick invokes the action, the merchant is navigated to your app at the configured `url`

### Customizing the intent

1. Update the `type` and `action` in `shopify.extension.toml` to match your app's capabilities
2. Update `intent-schema.json` with the input parameters your action accepts
3. Update the `url` in `shopify.extension.toml` to point to the correct page in your app
4. Update `tools.json` with the tools Sidekick can use for your action
5. Update `instructions.md` with guidelines for when and how Sidekick should use your tools

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
