#!/usr/bin/env bash
# Kotoba install script — placeholder.
#
# Real install will live at https://kotoba.dev/install.sh once releases ship.
# For now, build from source:
#
#   git clone https://github.com/your-org/kotoba.git
#   cd kotoba
#   cargo install --path crates/kotoba-cli
#
# This script is a stub that prints those instructions.
set -euo pipefail

cat <<'EOF'
Kotoba is in early alpha. To install from source:

  git clone https://github.com/your-org/kotoba.git
  cd kotoba
  cargo install --path crates/kotoba-cli

Or, to try without installing:

  cargo run -p kotoba-cli -- init
  cargo run -p kotoba-cli -- lookup ありがとう

A real release pipeline (Homebrew, single-binary downloads) will land with v0.1.
See docs/roadmap.md for the timeline.
EOF
