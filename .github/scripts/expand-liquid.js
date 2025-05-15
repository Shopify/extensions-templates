/***
 * For use without the Shopify CLI only.
 * This script replaces the liquid expansion functionality of the Shopify CLI for use cases where the Shopify CLI is not being used.
 */

import glob from "fast-glob";
import { Liquid } from "liquidjs";
import path from "path";
import fs from "node:fs/promises";
import { exec } from "child_process";

async function expandLiquidTemplates(template, liquidData) {
  const entries = await glob([path.join(template, "**/*.liquid")], {
    dot: true,
    ignore: ["**/node_modules"],
  });

  for (const entry of entries) {
    const engine = new Liquid();
    const rendered = await engine.renderFile(entry, liquidData);
    const outputPath = entry.replace(".liquid", "");
    await fs.writeFile(outputPath, rendered);
    console.log(`  ${path.relative(process.cwd(), outputPath)}`);
  }
}

async function directoryNames(parentPath) {
  return (await fs.readdir(parentPath, { withFileTypes: true }))
    .filter((dirent) => dirent.isDirectory())
    .map((dirent) => dirent.name);
}

async function expandExtensionLiquidTemplates(projectName, flavor) {
  console.log(`Expanding liquid templates for ${projectName}`);
  const pathSuffix =
    flavor === "typescript" || flavor === "vanilla-js" ? "js" : "rs";
  const projectPath = path.join(process.cwd(), projectName + "-" + pathSuffix);
  const langName =
    flavor === "typescript" || flavor === "vanilla-js" ? "javascript" : "rust";

  if (langName === "javascript") {
    await (
      await glob(path.join(projectPath, "src", "!(*.liquid|*.graphql)"))
    ).forEach(async (path) => await fs.rm(path));
  }

  const liquidData = {
    name: `${projectName}`,
    handle: `${projectName}`,
    flavor,
  };

  await expandLiquidTemplates(projectPath, liquidData);

  if (langName === "javascript") {
    const srcFilePaths = await glob(
      path.join(projectPath, "src", "!(*.liquid|*.graphql)")
    );
    const srcFileExtensionsToChange = [];

    const fileExtension = flavor === "typescript" ? "ts" : "js";

    for (const srcFilePath of srcFilePaths) {
      srcFileExtensionsToChange.push(
        fs.rename(srcFilePath, `${srcFilePath}.${fileExtension}`, (err) => {
          if (err) throw err;
        })
      );
    }

    await Promise.all(srcFileExtensionsToChange);
  }

  console.log();
}

function ensureNoGitChanges() {
  exec("git status --porcelain", (error, stdout, _stderr) => {
    if (error) {
      console.error(`error calling \`git status\`: ${error}`);
      process.exit(1);
    }
    if (stdout) {
      console.error("Untracked files detected:\n", stdout);
      exec("git diff", (error, stdout, _stderr) => {
        if (error) {
          console.error(`error calling \`git diff\`: ${error}`);
        } else {
          console.log(`Git diff:\n${stdout}`);
        }
        process.exit(1);
      });
    }
  });
}

const flavor = process.argv[2] || "vanilla-js";
const project = process.argv[3] || null;

if (project) {
  await expandExtensionLiquidTemplates(project, flavor);
} else {
  const projects = await directoryNames(process.cwd());
  const projectsToExpand = [
    ...new Set(
      projects
        .filter((project) => project.startsWith("functions-"))
        .map((project) => project.substring(0, project.lastIndexOf("-")))
    ),
  ];
  for (const project of projectsToExpand) {
    await expandExtensionLiquidTemplates(project, flavor);
  }
}
console.log(
  "The above files should be added to .gitignore if they have not already been added.\n"
);

if (process.env.CI) {
  ensureNoGitChanges();
}
