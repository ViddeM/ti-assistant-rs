map_renderer:
	cd backend/ && \
	cargo build --release --target wasm32-unknown-unknown --bin ti_helper_map_render && \
	wasm-bindgen --target web --out-dir ./map_render/out/ --out-name "map_render" ./target/wasm32-unknown-unknown/release/ti_helper_map_render.wasm && \
	cp -r map_render/out/* ../frontend/public/map_render/game/
