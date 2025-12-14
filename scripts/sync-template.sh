\#!/bin/bash
echo "Syncing template improvements..."
git fetch template
git merge template/main --allow-unrelated-histories
cargo run --bin registry-tool
echo "Done! Resolve any conflicts in yearXXXX/mod.rs if needed."
