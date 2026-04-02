# Admin App Intent Render Extension

Admin app intent render extensions let app developers render custom UI when Sidekick invokes an app action in Shopify Admin. Use this extension to review the intent payload, collect any final confirmation from the merchant, and resolve the workflow back to Sidekick.

Learn more about app action extensions in Shopify's [developer documentation](https://shopify.dev/docs/apps/build/sidekick/build-app-actions).

---

## Get started with this extension

This extension demonstrates rendering an email compose action for Sidekick. After deployment, Sidekick can invoke the intent, pass the input payload into your extension, and wait for your extension to resolve the workflow.

### Key files

- `src/IntentExtension.jsx` - Main extension code that reads the current intent payload and resolves the workflow
- `shopify.extension.toml` - Extension configuration defining the render target and registered intent
- `intent-schema.json` - Schema definition for the intent input
- `tools.json` - Tool definitions available while Sidekick is working in this intent
- `instructions.md` - Guidance for when Sidekick should use your tools

### How it works

1. The extension registers an app action with `[[extensions.targeting.intents]]`
2. When Sidekick invokes that action, Shopify renders this extension at `admin.app.intent.render`
3. The extension reads the current intent payload from the URL, shows it to the merchant, and resolves the workflow with `shopify.intents.response`

### Customizing the action

1. Update `type` and `action` in `shopify.extension.toml` to match your app's action
2. Update `intent-schema.json` to match the input your extension expects
3. Replace the example UI in `src/IntentExtension.jsx` with your app's workflow
4. Update `tools.json` and `instructions.md` to match the tools Sidekick can use in this flow

### Testing

Run `shopify app deploy` to deploy your extension and test it with Sidekick in Shopify Admin.
