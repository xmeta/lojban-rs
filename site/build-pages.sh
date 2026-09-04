#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST="${1:-$ROOT/site/dist}"
MANIFEST="$ROOT/site/wasm/Cargo.toml"
TARGET="$ROOT/site/wasm/target/wasm32-unknown-unknown/release/lojban_web.wasm"

if ! command -v wasm-bindgen >/dev/null 2>&1; then
  echo "wasm-bindgen CLI is required" >&2
  exit 1
fi

rm -rf "$DIST"
mkdir -p "$DIST/pkg"

cargo build \
  --manifest-path "$MANIFEST" \
  --target wasm32-unknown-unknown \
  --release

wasm-bindgen "$TARGET" \
  --target web \
  --out-dir "$DIST/pkg" \
  --no-typescript
cp "$ROOT/examples/web_playground/index.html" "$DIST/index.html"
cp "$ROOT/examples/web_playground/style.css" "$DIST/style.css"
cp "$ROOT/examples/web_playground/runtime.js" "$DIST/runtime.js"
cp "$ROOT/examples/web_playground/app.js" "$DIST/app.js"

python3 - "$DIST/index.html" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
old = '<meta name="lojban-runtime" content="server">'
new = '<meta name="lojban-runtime" content="wasm">'
if old not in text:
    raise SystemExit('runtime meta tag not found')
path.write_text(text.replace(old, new, 1))
PY

touch "$DIST/.nojekyll"
printf 'Built GitHub Pages site: %s\n' "$DIST"
