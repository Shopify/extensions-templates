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
scopes = ""
extension_directories = ["${EXT_DIR_PATTERN}"]
EOF

for dir in $EXT_DIR_PATTERN; do
  echo "Running wasm tests in $dir"
  (cd "$dir" && npx vitest run)
done
