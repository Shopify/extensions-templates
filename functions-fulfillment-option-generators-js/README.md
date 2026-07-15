# Fulfillment option generators demo

This repository contains a function that demonstrates how to generate fulfillment options for the fulfillments in a
cart, based on an external API accessible via an HTTP request. To simulate an external API, we have hosted a
[JSON file](https://cdn.shopify.com/s/files/1/0628/3830/9033/files/pickup-points-external-api-v2.json?v=1714588690),
which contains delivery point information in the following format:

```json
{
  "deliveryPoints": [
    {
      "pointId": "001",
      "pointName": "Toronto Store",
      "location": {
        "addressComponents": {
          "streetNumber": "620",
          "route": "King St W",
          "locality": "Toronto",
          "administrativeArea": {
            "name": "Ontario",
            "code": "ON"
          },
          "postalCode": "M5V 1M6",
          "country": "Canada",
          "countryCode": "CA"
        },
        "geometry": {
          "location": {
            "lat": 43.644664618786685,
            "lng": -79.40066267417106
          }
        }
      },
      "openingHours": {
        "weekdayText": [
          "Monday: 9:00 AM – 9:00 PM",
          "Tuesday: 9:00 AM – 9:00 PM",
          "Wednesday: 9:00 AM – 9:00 PM",
          "Thursday: 9:00 AM – 9:00 PM",
          "Friday: 9:00 AM – 9:00 PM",
          "Saturday: 10:00 AM – 6:00 PM",
          "Sunday: Closed"
        ]
      }
    }
  ]
}
```

## Implementation details

A function can have one or more targets, each characterized by a specific input/output API. The Fulfillment Option
Generators have two targets: an optional **fetch** target and a **run** target. The input/output APIs are
represented as a GraphQL API within the attached [schema](./schema.graphql).

### Fetch target

The **fetch** target is responsible for generating an HTTP request to call the external API. Its input API is defined
by the `Input` type in the [schema](./schema.graphql). In our demo, we are only interested in the buyer's localization
country, which we specify within the [**fetch** target input query](./src/fetch.graphql).

The [**fetch** target](./src/fetch.js) reads the input and generates an output representing an HTTP request to the
external API if the buyer's country is Canada. The output API is defined by the `CartFulfillmentOptionsGenerateFetchResult` type in
the [schema](./schema.graphql).

#### Fetch target input/output example

##### Input

```json
{
  "localization": {
    "country": {
      "isoCode": "CA"
    }
  }
}
```

##### Output

```json
{
  "request": {
    "method": "GET",
    "url": "https://cdn.shopify.com/s/files/1/0628/3830/9033/files/pickup-points-external-api-v2.json?v=1714588690",
    "headers": [
      {
        "name": "Accept",
        "value": "application/json; charset=utf-8"
      }
    ],
    "body": null,
    "policy": {
      "readTimeoutMs": 500
    }
  }
}
```

### Run target

The **run** target is responsible for generating the fulfillment options. Its input API is defined by the `Input` type
in the [schema](./schema.graphql). In our demo, we are interested in the cart's fulfillments along with the external
API HTTP response status and body, which we specify within the [**run** target input query](./src/run.graphql).

The [**run** target](./src/run.js) parses the response body and produces fulfillment options in the format specified
by the `CartFulfillmentOptionsGenerateRunResult` type in the [schema](./schema.graphql). Providers and service points are deduplicated into
the `references` block and referenced from `fulfillmentOptionsAdd` operations by their handle, to keep the payload
small. One fulfillment option is generated per service point, for every input fulfillment.

#### Run target input/output example

##### Input

```json
{
  "fulfillments": [
    { "handle": "fulfillment-1" }
  ],
  "fetchResult": {
    "status": 200,
    "body": "{\"deliveryPoints\":[{\"pointId\":\"001\",\"pointName\":\"Toronto Store\",\"location\":{\"addressComponents\":{\"streetNumber\":\"620\",\"route\":\"King St W\",\"locality\":\"Toronto\",\"administrativeArea\":{\"name\":\"Ontario\",\"code\":\"ON\"},\"postalCode\":\"M5V 1M6\",\"country\":\"Canada\",\"countryCode\":\"CA\"},\"geometry\":{\"location\":{\"lat\":43.644664618786685,\"lng\":-79.40066267417106}}},\"openingHours\":{\"weekdayText\":[\"Monday: 9:00 AM – 9:00 PM\",\"Sunday: Closed\"]}}]}"
  }
}
```

##### Output

```json
{
  "references": {
    "providers": [
      {
        "handle": "shopify-demo-provider",
        "externalId": "shopify-demo",
        "name": "Shopify Functions Demo",
        "logoUrl": "https://cdn.shopify.com/s/files/1/0628/3830/9033/files/shopify_icon_146101.png?v=1706120545"
      }
    ],
    "servicePoints": [
      {
        "handle": "service-point-001",
        "externalId": "001",
        "name": "Toronto Store",
        "address1": "620 King St W",
        "address2": null,
        "city": "Toronto",
        "countryCode": "CA",
        "provinceCode": "ON",
        "zip": "M5V 1M6",
        "phone": null,
        "latitude": 43.644664618786685,
        "longitude": -79.40066267417106,
        "businessHours": [
          {
            "day": "MONDAY",
            "periods": [
              {
                "openingTime": "09:00:00",
                "closingTime": "21:00:00"
              }
            ]
          },
          {
            "day": "SUNDAY",
            "periods": []
          }
        ]
      }
    ]
  },
  "operations": [
    {
      "fulfillmentOptionsAdd": {
        "fulfillmentHandle": "fulfillment-1",
        "title": "Toronto Store",
        "providerHandle": "shopify-demo-provider",
        "destinationServicePointHandle": "service-point-001",
        "cost": null,
        "metafields": []
      }
    }
  ]
}
```

## Usage

### Installing dependencies

1. Install the necessary dependencies by running the following command in your terminal:

```bash
yarn install
```

### Running tests

1. Execute the tests by running the following command in your terminal:

```bash
yarn test
```

### Deploying the function to the app

1. Navigate to the root directory of your app. Deploy the function by running the following command
in your terminal:

```bash
yarn deploy
```

### Using the function in a store

1. Activate the function by selecting it in the relevant fulfillment settings of the store admin, then save.

2. To use the function, initiate a checkout process with a product available from the configured location. Enter an
address in Canada — a list of fulfillment options generated using this function should now be visible.
