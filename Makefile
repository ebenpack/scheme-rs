default: web

.PHONY: clean-www
clean-www:
	rm -rf www/dist
	rm -rf www/node_modules
	rm -rf npm-build

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
	cargo build --workspace --release

.PHONY: wasm_build
wasm_build: clean cargo-build
	npx wasm-pack build --release

.PHONY: web
web: wasm_build
	yarn --cwd www install --frozen-lockfile
	yarn --cwd www build

.PHONY: profile
profile:
	cargo build --workspace  --profile profiling
	samply ./target/profiling/scheme-rs-wasm ~/Desktop/micro-kanren-test.scm                                                                 ─╯

.PHONY: npm-package
npm-package: wasm_build
	$(eval current_git_url := $(shell git ls-remote --get-url origin))
	mkdir -p npm-build
	git init npm-build/
	git -C npm-build/ remote add origin ${current_git_url}
	git -C npm-build/ fetch
	git -C npm-build/ checkout build
	cp -a pkg/. npm-build/
	# wasm-pack creates a gitignore with `*`. We want to upload the contents
	# to our build branch, and we already ignore pkg/ so we'll delete this.
	rm npm-build/.gitignore
	git -C npm-build/ add .
	git -C npm-build/ commit -m "New build - $(shell date "+%Y-%m-%d %H:%M:%S")"
	git -C npm-build/ push --set-upstream origin build

.PHONY: serve
serve:
	python3 -m http.server 8000 --directory www/dist