default: web

.PHONY: clean-www
clean-www:
	rm -rf www/dist
	rm -rf www/node_modules

.PHONY: clean-rs
clean-rs:
	rm -rf pkg

.PHONY: clean
clean: clean-www clean-rs

.PHONY: test
test:
	cargo test --workspace

.PHONY: cargo-build
cargo-build:
	cargo build

.PHONY: web
web:
	npx --yes -c 'wasm-pack build --release'
	yarn --cwd www install --frozen-lockfile 
	yarn --cwd www build

.PHONY: serve
serve:
	python3 -m http.server 8000 --directory www/dist