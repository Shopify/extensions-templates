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
    const schemaPath = path.join(functionDir, "schema.graphql");
    const inputQueryPath = path.join(functionDir, "src/run.graphql");
    schema = await loadSchema(schemaPath);
    inputQueryAST = await loadInputQuery(inputQueryPath);
  }, 30000);

  const fixturesDir = path.join(__dirname, "fixtures");
  const fixtureFiles = fs.readdirSync(fixturesDir)
    .filter((file) => file.endsWith(".json"))
    .map((file) => path.join(fixturesDir, file));

  fixtureFiles.forEach((fixtureFile) => {
    test(`runs ${path.relative(fixturesDir, fixtureFile)}`, async () => {
      const fixture = await loadFixture(fixtureFile);
      const validationResult = await validateTestAssets({ schema, fixture, inputQueryAST });
      expect(validationResult.inputQuery.valid).toBe(true);
      expect(validationResult.fixtureInputTypes.valid).toBe(true);
      expect(validationResult.fixtureInputStructure.valid).toBe(true);
      expect(validationResult.fixtureOutput.valid).toBe(true);
      const runResult = await runFunction(fixture.export, fixture.input, functionDir);
      const { result, error } = runResult;
      expect(error).toBeNull();
      expect(result.output).toEqual(fixture.expectedOutput);
    }, 10000);
  });
});
