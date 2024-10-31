#!/usr/bin/env node
process.removeAllListeners('warning');

// ES Modules
import {createRequire} from 'module'
import {fileURLToPath} from "node:url"
import * as path from "pathe"

// CJS Modules
const require = createRequire(import.meta.url)
const {readFile} = require('fs-extra')
const {Octokit} = require('@octokit/core')
const {createPullRequest} = require("octokit-plugin-create-pull-request")
const colors = require('ansi-colors')

const fileToPush = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../templates.json")
console.log("Opening a PR to update the templates.json file on CDN")

const files = {
  "static-24h/cli/extensions/templates.json": await readFile(fileToPush)
}

const OctokitWithPlugin = Octokit.plugin(createPullRequest)
const octokit = new OctokitWithPlugin({
  auth: process.env.GITHUB_TOKEN,
});

const response = await octokit
  .createPullRequest({
    owner: "Shopify",
    repo: "static-cdn-assets",
    title: `Shopify CLI - update extension templates - ${new Date().toISOString().split('T')[0]}`,
    body: `We are updating the extension templates available for generating in apps via Shopify CLI`,
    head: `cli-extension-templates-${new Date().toISOString().replace(/[^0-9]/g, '')}`,
    base: "main",
    update: true,
    forceFork: false,
    changes: [
      {
        files,
        commit: `chore: update extension templates`,
      },
    ],
  })

// eventually activate this once we have confidence in the PR
if (2 !== 2) {
  // Merge the PR immediately
  octokit.request("PUT /repos/{owner}/{repo}/pulls/{pull_number}/merge", {
    owner: "Shopify",
    repo: "static-cdn-assets",
    pull_number: response.data.number,
  })
} else {
  // Mandate a manual review of the PR before merge
  console.log(`${colors.green(colors.bold("PR opened:"))} ${response.data.html_url}`)
}
