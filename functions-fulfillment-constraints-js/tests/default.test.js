import path from "path";
import fs from "fs";
import { describe, beforeAll, test, expect } from "vitest";
import { buildFunction, loadSchema, loadInputQuery, loadFixture, validateTestAssets, runFunction } from "@shopify/shopify-function-test-helpers";

describe("Default Integration Test", () => {
  let schema;
  let inputQueryAST;
  let functionDir;

  beforeAll(async () => {
    functionDir = path.dirname(__dirname);
    await buildFunction(functionDir);

    // Load schema and input query once since they don't change across fixtures
    const schemaPath = path.join(functionDir, "schema.graphql");
    const inputQueryPath = path.join(functionDir, "src/cart_fulfillment_constraints_generate_run.graphql");

    schema = await loadSchema(schemaPath);
    inputQueryAST = await loadInputQuery(inputQueryPath);
  }, 30000); // 30 second timeout for building the function

  const fixturesDir = path.join(__dirname, "fixtures");
  const fixtureFiles = fs
    .readdirSync(fixturesDir)
    .filter((file) => file.endsWith(".json"))
    .map((file) => path.join(fixturesDir, file));

  fixtureFiles.forEach((fixtureFile) => {
    test(`runs ${path.relative(fixturesDir, fixtureFile)}`, async () => {
      const fixture = await loadFixture(fixtureFile);

      // Validate fixture using our comprehensive validation system
      const validationResult = await validateTestAssets({
        schema,
        fixture,
        inputQueryAST
      });

      // Assert that all validation steps pass
      expect(validationResult.inputQuery.valid).toBe(true);
      expect(validationResult.inputFixture.valid).toBe(true);
      expect(validationResult.inputQueryFixtureMatch.valid).toBe(true);
      expect(validationResult.outputFixture.valid).toBe(true);

      // Run the actual function
      const runResult = await runFunction(
        fixture.export,
        fixture.input,
        functionDir
      );

      const { result, error } = runResult;
      expect(error).toBeNull();
      expect(result.output).toEqual(fixture.expectedOutput);
    }, 10000);
  });
});
