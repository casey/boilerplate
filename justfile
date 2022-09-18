watch +args='test --workspace --all-targets':
  cargo watch --clear --exec '{{args}}'

ci:
  cargo test --workspace --all-targets
  cargo test --workspace --all-targets --features axum
  cargo clippy --workspace --all-targets
  cargo fmt --workspace -- --check

# publish current GitHub master branch
publish:
  #!/usr/bin/env bash
  set -euxo pipefail
  rm -rf tmp/release
  git clone git@github.com:casey/boilerplate.git tmp/release
  VERSION=`sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
  cd tmp/release
  git tag -a $VERSION -m "Release $VERSION"
  git push origin $VERSION
  cargo publish
  cd ../..
  rm -rf tmp/release
