watch +args='test':
  cargo watch --clear --exec '{{args}}'

ci:
  cargo test --all --all-targets
  cargo test --all --all-targets --features axum
  cargo clippy --all --all-targets
  cargo fmt --all -- --check

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
