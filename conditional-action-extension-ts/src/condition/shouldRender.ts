// The second argument to the render callback provides access to the resource ID.
export default async function extension() {
  const {data} = shopify;
  const variantCount = await getVariantsCount(data.selected[0].id);

  return {display: variantCount > 1 }
};

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
    variables: {id},
  };

  // Temporary until 2025-07 is available
  const res = await fetch("shopify:admin/api/2025-04/graphql.json", {
    method: "POST",
    body: JSON.stringify(getProductQuery),
  });

  if (!res.ok) {
    console.error("Network error");
  }

  const productData = await res.json();
  return productData.data.product.variantsCount.count;
}
