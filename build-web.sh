#!/bin/bash

# cargo build --release --target wasm32-unknown-unknown

# if ! command -v wasm-opt >/dev/null; then
#     echo "\"wasm-opt\" is not installed, skip optimize."
#     cp target/wasm32-unknown-unknown/release/sbchild-blog.wasm sbchild-blog-release.wasm
# else
#     echo "Optimizing with \"wasm-opt\"..."
#     wasm-opt -Os --output sbchild-blog-release.wasm target/wasm32-unknown-unknown/release/sbchild-blog.wasm
# fi

# origin_str=$(du -h target/wasm32-unknown-unknown/release/sbchild-blog.wasm | awk '{print $1}')
# minify_str=$(du -h sbchild-blog-release.wasm | awk '{print $1}')

# echo "$origin_str -> $minify_str"
cd "$(git rev-parse --show-toplevel)" || exit 3
cd "target_web" || exit 4
echo "Current dir is $PWD"

if ! command -v wasm-pack >/dev/null; then
    echo "Please install \`wasm-pack\` first."
    exit 1
fi

CARGO_PROFILE_RELEASE_OPT_LEVEL=s CARGO_PROFILE_RELEASE_STRIP=debuginfo CARGO_PROFILE_RELEASE_LTO=true \
    wasm-pack build --target web || (cd "$(git rev-parse --show-toplevel)" && exit 2)

wasm_size=$(du -h pkg/sbchild_blog_web_bg.wasm | awk '{print $1}')

echo "Release size: $wasm_size"

cd "$(git rev-parse --show-toplevel)" || exit 5

exit 0
