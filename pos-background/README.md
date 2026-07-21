# POS background extension

Scaffold for a non-visual POS background extension targeting `pos.app.ready.data`.

The background target runs for the full POS session and observes events via
`shopify.addEventListener()`. It does not render UI. See the example handlers in
`src/background-extension.{js,ts}` for `transactioncomplete`,
`cashtrackingsessionstart`, and `cashtrackingsessioncomplete`.
