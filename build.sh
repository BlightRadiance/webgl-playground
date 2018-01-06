echo "Building to wasm"
# cargo +nightly build --target wasm32-unknown-unknown --release
cargo-web build --target-webasm

echo "Moving wasm file"
mv -f "target\wasm32-unknown-unknown\release\wasm-test.wasm" dist
mv -f "target\wasm32-unknown-unknown\release\wasm-test.js" dist

echo "Minify wasm output"
wasm-gc dist/wasm-test.wasm dist/wasm-test.wasm