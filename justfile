watch +args='check --all':
  cargo watch --clear --exec '{{args}}'

ci:
  cargo clippy --all --all-targets -- --deny warnings
  cargo fmt --all -- --check
  cargo test --all
  cargo test --all --features axum
  cargo test --all --features reload
  cargo build --target thumbv6m-none-eabi --package boilerplate
  cargo build --target thumbv6m-none-eabi --package boilerplate-tests

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
  cargo publish --workspace
  cd ../..
  rm -rf tmp/release

outdated:
  cargo outdated --root-deps-only --workspace

unused:
  cargo +nightly udeps --workspace
