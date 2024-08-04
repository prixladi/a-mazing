cd "$(dirname "$0")"
wasm-pack build --target web --out-name mazer --out-dir ./pkg
cd -