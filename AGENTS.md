# Agent Guide

## Repository Overview

Mono is a schema-first repository for the Oryon API surface. Protocol Buffer definitions live under `proto/` and are used with Buf to generate client/server code for Go, Rust, and TypeScript.

## Repository Layout

- `proto/` — Protocol Buffer source definitions.
- `gen/go/` — Generated Go protobuf, gRPC, and ConnectRPC code.
- `gen/rust/` — Generated Rust protobuf and ConnectRPC code.
- `gen/ts/` — Generated TypeScript code.
- `scaffold/` — Templates used during code generation.
- `buf.yaml` / `buf.gen.yaml` — Buf module, linting, breaking-change, and generation configuration.
- `Makefile` — Canonical generation, validation, and publishing commands.
- `.github/workflows/` — CI configuration.

## Development Rules

1. Treat `proto/` as the source of truth. Do not hand-edit generated files under `gen/`.
2. After changing protobuf definitions, run `make gen` and commit the resulting generated changes.
3. Preserve Buf lint and breaking-change requirements unless a breaking change is intentional.
4. Keep generated output reproducible from the checked-in Buf configuration and scaffold files.
5. Keep changes focused and avoid unrelated generated-file churn.

## Validation

```sh
make gen
make check
```

For linting only:

```sh
make lint
```

`make check` runs Buf linting, builds the generated Go packages, and runs `cargo check` for the generated Rust crate.

## Publishing

Publishing is guarded by the Makefile and requires a clean working tree:

```sh
make publish TAG=0.1.0
```

Do not publish or create release tags unless explicitly requested.

## Generated Code

Buf generates:

- Go protobuf, gRPC, and ConnectRPC code
- Rust protobuf and ConnectRPC code
- TypeScript protobuf/ES code

After modifying `proto/`, regenerate instead of editing `gen/` manually.
