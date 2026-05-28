#!/bin/bash
# Loopt alle demo-stappen achter elkaar door.
# Werkt zowel in een lokale shell als in de Docker-container.

set -u
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

sep() { echo ""; echo "============================================================"; echo "$1"; echo "============================================================"; }

sep "Stap 1 — C zonder sanitizer (de bug is zichtbaar in de output)"
cd "$SCRIPT_DIR/c"
make run

sep "Stap 2 — C met AddressSanitizer (runtime-detectie)"
make asan || true

sep "Stap 3 — Rust met dezelfde bug (compileert NIET)"
cd "$SCRIPT_DIR/rust/broken"
cargo build || true

sep "Stap 4 — Rust correct (werkt)"
cd "$SCRIPT_DIR/rust/fixed"
cargo run

echo ""
echo "Klaar. Zelfde bug-klasse. Twee talen. Twee uitkomsten."
