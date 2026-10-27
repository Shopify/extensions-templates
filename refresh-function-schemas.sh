#!/usr/bin/env bash
#
# refresh-function-schemas.sh — refresh schema.graphql for function templates.
#
# For every (function type, flavor) pair in templates.json, symlinks the
# local template dir into a linked Shopify app's `extensions/` directory
# and runs `shopify app function schema` to fetch the schema for the
# api_version declared in that template's toml. Each flavor's schema is
# fetched independently so the flavors can drift in api_version without
# one overwriting another.
#
# Usage:
#   ./refresh-function-schemas.sh --app-dir <path> [options]
#
# Required:
#   --app-dir <path>     Linked Shopify app directory (must have shopify.app.toml)
#
# Options:
#   --only <id[,id...]>  Refresh only these function identifiers.
#                        Default: every function in templates.json.
#   --keep               Leave generated extension symlinks in the app.
#   -h, --help           Show this help.
#
# Operates on the extensions-templates checkout containing this script.
#
# Requirements: bash, git, jq, yarn, shopify CLI (@shopify/cli).
#
# Example:
#   ./refresh-function-schemas.sh --app-dir ~/src/my-dev-app
#   ./refresh-function-schemas.sh --app-dir ~/src/my-dev-app --only cart_transform,discount
#
set -euo pipefail

TEMPLATES_REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_DIR=""
ONLY=""
KEEP=0

usage() { sed -n '2,/^set -euo/p' "$0" | sed 's/^# \{0,1\}//; $d'; exit "${1:-0}"; }

while [ $# -gt 0 ]; do
  case "$1" in
    --app-dir) APP_DIR="$2"; shift 2 ;;
    --only)    ONLY="$2"; shift 2 ;;
    --keep)    KEEP=1; shift ;;
    -h|--help) usage 0 ;;
    *) echo "Unknown arg: $1" >&2; usage 1 ;;
  esac
done

[ -n "$APP_DIR" ] || { echo "--app-dir is required" >&2; usage 1; }
[ -f "$APP_DIR/shopify.app.toml" ] || { echo "No shopify.app.toml in $APP_DIR — is it linked?" >&2; exit 1; }
[ -f "$TEMPLATES_REPO/templates.json" ] || { echo "No templates.json in $TEMPLATES_REPO" >&2; exit 1; }
command -v jq >/dev/null || { echo "jq is required" >&2; exit 1; }
command -v yarn >/dev/null || { echo "yarn is required" >&2; exit 1; }
command -v shopify >/dev/null || { echo "shopify CLI is required" >&2; exit 1; }

only_match() {
  [ -z "$ONLY" ] && return 0
  case ",$ONLY," in *",$1,"*) return 0 ;; esac
  return 1
}

echo "Expanding liquid templates..."
for flavor in rust vanilla-js wasm; do
  ( cd "$TEMPLATES_REPO" && yarn --silent expand-liquid "$flavor" >/dev/null )
done

ROWS=()
while IFS= read -r line; do
  ROWS+=("$line")
done < <(
  jq -r '
    .[]
    | select(.type == "function") as $t
    | [$t.supportedFlavors[].path] | unique | .[] as $p
    | [$t.identifier, $p] | @tsv
  ' "$TEMPLATES_REPO/templates.json"
)

cleanup_list=()
cleanup() {
  [ "$KEEP" = 1 ] && return
  for ext in "${cleanup_list[@]}"; do
    rm -f "$APP_DIR/extensions/$ext"
  done
}
trap cleanup EXIT

mkdir -p "$APP_DIR/extensions"

refreshed=0
skipped=0
for row in "${ROWS[@]}"; do
  ID="$(printf '%s' "$row" | cut -f1)"
  REL_PATH="$(printf '%s' "$row" | cut -f2)"
  [ -n "$ID" ] && [ -n "$REL_PATH" ] || continue

  if ! only_match "$ID"; then
    skipped=$((skipped + 1)); continue
  fi

  TEMPLATE_DIR="$TEMPLATES_REPO/$REL_PATH"
  if [ ! -f "$TEMPLATE_DIR/shopify.extension.toml" ]; then
    echo "SKIP $REL_PATH (no expanded toml)"
    skipped=$((skipped + 1)); continue
  fi

  EXT="schemagen-$REL_PATH"
  LINK="$APP_DIR/extensions/$EXT"
  rm -rf "$LINK"
  ln -s "$TEMPLATE_DIR" "$LINK"
  cleanup_list+=("$EXT")

  echo ">> $REL_PATH — fetching schema"
  ( cd "$APP_DIR" && shopify app function schema --path "$LINK" )

  refreshed=$((refreshed + 1))
done

echo
echo "Done. Refreshed: $refreshed, skipped: $skipped."
echo "Review: (cd $TEMPLATES_REPO && git status)"
