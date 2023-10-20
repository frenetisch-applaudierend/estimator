#!/usr/bin/env bash

set +e

root_dir=$(dirname $(realpath $0))

trunk build "$root_dir/packages/client/index.html" \
  --release \
  --dist "$root_dir/dist"

cargo build --bin server