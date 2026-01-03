# To build and add to the website
Ensure wasm-bindgen etc are installed

Steps:
 - Build for wasm target: `cargo build --release --target wasm32-unknown-unknown`
 - Create wasm bindings: `wasm-bindgen --target web --out-dir ./out ../target/wasm32-unknown-unknown/release/ti_helper_map_render.wasm`
 - Move files to frontend `out/* ../../frontend/public/map_render/game`
