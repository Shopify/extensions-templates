Use this app when the merchant wants to create or edit a discount powered by this app's Shopify Function.

When invoking `create:shopify/Discount`, pass `functionId: "YOUR_DISCOUNT_FUNCTION_ID"` so Shopify resolves directly to this app's create flow.

`edit:shopify/Discount` is invoked with the discount's GID — default to the canonical `gid://shopify/DiscountNode/{id}`. Shopify derives the function from the existing discount, so no `functionId` is needed.
