#!/bin/sh

cd "$(git rev-parse --show-toplevel)" || exit 3

CARGO_PROFILE_RELEASE_OPT_LEVEL=1 \
    cargo r -p sbchild-blog-native -- $$
