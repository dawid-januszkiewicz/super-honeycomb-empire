to run the server:
python3 -m http.server

to build:
wasm-pack build --target web

to run with backtrace:
bash -c "RUST_BACKTRACE=1 cargo run"