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

.PHONY: wasm_build
wasm_build: clean
	npx wasm-pack build --release

.PHONY: web
web: wasm_build
	yarn --cwd www install --frozen-lockfile
	yarn --cwd www build


.PHONY: npm-package
npm-package: wasm_build
	$(eval current_git_url := $(shell git ls-remote --get-url origin))
	git init pkg/
	git -C pkg/ config remote.origin.url >&- || git -C pkg/ remote add origin ${current_git_url}
	git -C pkg/ checkout build || git -C pkg/ checkout --orphan build
	# wasm-pack creates a gitignore with `*`. We want to upload the contents
	# to our build branch, and we already ignore pkg/ so we'll delete this.
	rm pkg/.gitignore
	git -C pkg/ add .
	git -C pkg/ commit -m "New build - $(shell date "+%Y-%m-%d %H:%M:%S")"
	git -C pkg/ push --force --set-upstream origin build

.PHONY: serve
serve:
	python3 -m http.server 8000 --directory www/dist