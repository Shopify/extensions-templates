// The target used here must match the target used in the extension's toml file (./shopify.extension.toml), 
// except for the "should-render" suffix
const TARGET = 'admin.product-details.action.should-render';

// The second argument to the render callback provides access to the resource ID.
export default shopify.extend(TARGET, async ({ data }) => {
  const variantCount = await getVariantsCount(data.selected[0].id);

  return {display: variantCount > 1 }
});

// Use direct API calls to fetch data from Shopify.
// See https://shopify.dev/docs/api/admin-graphql for more information about Shopify's GraphQL API
async function getVariantsCount(id) {
  const getProductQuery = {
    query: `query Product($id: ID!) {
      product(id: $id) {
        variantsCount {
          count
        }
      }
    }`,
    variables: { id },
  };

  const res = await fetch("shopify:admin/api/graphql.json", {
    method: "POST",
    body: JSON.stringify(getProductQuery),
  });

  if (!res.ok) {
    console.error("Network error");
  }

  const productData = await res.json();
  return productData.data.product.variantsCount.count;
}
