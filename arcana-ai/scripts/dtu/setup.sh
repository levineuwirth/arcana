#!/usr/bin/env bash
# setup.sh — prepare DTU's login node for an Arcana run at one commit.
#
#   ssh -o ControlPath=~/.ssh/cm-hpc hpc 'bash -s -- <commit>' < arcana-ai/scripts/dtu/setup.sh
#
# Installs rustup into the home if it is missing (no default toolchain; the
# repository's rust-toolchain.toml names one, installed on the first cargo
# call), clones or fetches the public repository into $ARCANA_REPO (default
# ~/Repos/arcana), checks out <commit> detached, and fetches every crate the
# lockfile names, so compute jobs build offline. Run on the login node: it
# downloads and does not compile. The commit must be pushed.
set -euo pipefail
commit="${1:?usage: setup.sh <commit>}"
repo="${ARCANA_REPO:-$HOME/Repos/arcana}"
export PATH="$HOME/.cargo/bin:$PATH"

if ! command -v rustup >/dev/null; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --profile minimal --default-toolchain none --no-modify-path
fi

if [ ! -d "$repo/.git" ]; then
  mkdir -p "$(dirname "$repo")"
  git clone --quiet https://github.com/levineuwirth/arcana.git "$repo"
fi
cd "$repo"
git fetch --quiet origin
git checkout --quiet --detach "$commit"
cargo fetch --locked
echo "ready: $(git rev-parse HEAD) with $(rustc --version)"
