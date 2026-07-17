Use this app when the merchant wants to create or edit a discount powered by this app's Shopify Function.

When invoking `create:shopify/Discount`, pass `functionId: "YOUR_DISCOUNT_FUNCTION_ID"` so Shopify resolves directly to this app's create flow.

When invoking `edit:shopify/Discount`, pass the discount's GID (default to the canonical `gid://shopify/DiscountNode/{id}`) and `functionId: "YOUR_DISCOUNT_FUNCTION_ID"` so Shopify resolves the edit directly to this app.
