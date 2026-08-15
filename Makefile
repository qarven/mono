.PHONY: gen lint check publish

TAG ?=

gen:
	@buf generate
	@cp scaffold/rust/Cargo.toml scaffold/rust/lib.rs gen/rust/oryon/
	@cp scaffold/ts/package.json gen/ts/
	@buf format proto -w

lint:
	@buf lint

check:
	@buf lint
	@go build ./gen/go/...
	@cargo check --manifest-path gen/rust/oryon/Cargo.toml

publish: check
	@test -n "$(TAG)" || { echo "usage: make publish TAG=0.1.0"; exit 1; }
	@test -z "$$(git status --porcelain)" || { echo "working tree not clean; commit first"; exit 1; }
	@make gen
	@test -z "$$(git status --porcelain)" || { echo "make gen changed files; commit them first"; exit 1; }
	@git push origin HEAD:main
	@git tag v$(TAG)
	@git push origin v$(TAG)