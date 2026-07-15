Use the `create_discount` action when the merchant asks to create a discount that your app powers with a discount function.

- This extension is pinned to a single discount function via the `functionId` `matchValue` in `intent-schema.json`. That discriminator handles intent routing, matching, and disambiguation, so the `create_discount` tool itself takes no input — don't pass the function ID as a tool argument.
- Invoke `create_discount` to open your app's discount creation flow for that function.
