# Mono

Mono is the schema and code-generation repository for the Oryon API surface.

## Overview

Protocol Buffer definitions are kept in one place and generate language-specific bindings for:

- Go
- Rust
- TypeScript

Generated artifacts are checked into `gen/`.

## Repository Structure

```text
.
├── proto/                 # Protocol Buffer definitions
│   └── oryon/
│       ├── authorization/
│       └── identity/
├── gen/
│   ├── go/                # Generated Go code
│   ├── rust/              # Generated Rust code/crate
│   └── ts/                # Generated TypeScript code
├── scaffold/              # Generation templates
├── buf.yaml               # Buf module and lint configuration
├── buf.gen.yaml            # Code-generation configuration
├── Makefile                # Development and release commands
└── .github/workflows/      # CI
```

## Requirements

- Go 1.27+
- Rust and Cargo
- Buf v1.73.0
- Git

The exact Buf plugins are configured in `buf.gen.yaml`.

## Development

Generate all bindings:

```sh
make gen
```

Lint:

```sh
make lint
```

Run the full checks:

```sh
make check
```

After changing a protobuf definition:

```sh
make gen
make check
git diff
```

Commit both the schema changes and regenerated output.

## Protobuf

The Buf module uses standard linting and file-level breaking-change checks. Dependencies include Google APIs and Protovalidate.

Current definitions live under:

```text
proto/oryon/authorization/
proto/oryon/identity/
```

## Release

The Makefile provides:

```sh
make publish TAG=0.1.0
```

Publishing requires a clean working tree and generated files that are already committed and unchanged by `make gen`.

## License

See [LICENSE](LICENSE).
