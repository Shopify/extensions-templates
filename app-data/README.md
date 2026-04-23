# Admin App Tools Extension

Admin app tools extensions enable app developers to provide data and search functionality to Sidekick in the Shopify Admin. These extensions allow Sidekick to query your app's external data sources and surface results to merchants.

Learn more about Admin app tools extensions in Shopify's [developer documentation](https://shopify.dev/docs/apps/build/sidekick/build-app-data).

---

## Get started with this extension

This extension demonstrates adding search functionality for Sidekick. After deployment, Sidekick will be able to run the search tool to query for the app's data.

### Key files

- `src/index.js` - Main extension code that defines the search tool execution logic
- `tools.json` - Schema definition for the search tool's inputs and outputs

### How it works

1. The extension registers a `search` tool using `shopify.tools.register()`
2. When Sidekick is asked to search for a resource, your search function is called with the query
3. Your function returns results matching the schema defined in `tools.json`

### Customizing the search

Edit `src/index.js` to implement your search logic:

1. Fetch data from your app's backend or API
2. Filter/search the data based on the `query` input
3. Return results in the expected format with pagination info

### Testing locally

Run `shopify app dev` and click on the "admin.app.tools.data" preview link in the Dev Console to test your extension in development mode
