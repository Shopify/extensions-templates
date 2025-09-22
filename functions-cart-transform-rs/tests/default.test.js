import path from "path";
import fs from "fs";
import { describe, beforeAll, test, expect } from "vitest";
import { buildFunction, loadFixture, runFunction, validateFixture } from "function-testing-helpers";

describe("Default Integration Test", () => {
  beforeAll(async () => {
    const functionDir = path.dirname(__dirname);
    await buildFunction(functionDir);
  }, 30000); // 30 second timeout for building the function

  const fixturesDir = path.join(__dirname, "fixtures");
  const fixtureFiles = fs
    .readdirSync(fixturesDir)
    .filter((file) => file.endsWith(".json"))
    .map((file) => path.join(fixturesDir, file));

  fixtureFiles.forEach((fixtureFile) => {
    test(`runs ${path.relative(fixturesDir, fixtureFile)}`, async () => {
      const fixture = await loadFixture(fixtureFile);

      const functionDir = path.dirname(__dirname);

      const schemaPath = path.join(functionDir, "schema.graphql");
      const inputQueryPath = path.join(functionDir, "src/cart_transform_run.graphql");

      await validateFixture(fixture);

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