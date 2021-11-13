default: web

.PHONY: clean-www
clean-www:
	rm -r www/dist

.PHONY: clean-rs
clean-rs:
	rm -r pkg

.PHONY: test
test:
	cargo test --workspace

.PHONY: web
web:
	wasm-pack build --release
	npm run build --prefix www