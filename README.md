# `::prettyplease-cli` or: How I learned to Stop Worrying and Trim the Comments.

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](
https://github.com/danielhenrymantilla/prettyplease-cli)
[![Latest version](https://img.shields.io/crates/v/prettyplease-cli.svg)](
https://crates.io/crates/prettyplease-cli)
[![MSRV](https://img.shields.io/badge/MSRV-1.58.0-white)](
https://gist.github.com/danielhenrymantilla/8e5b721b3929084562f8f65668920c33)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](
https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/crates/l/prettyplease-cli.svg)](
https://github.com/danielhenrymantilla/prettyplease-cli/blob/master/LICENSE-ZLIB)
[![CI](https://github.com/danielhenrymantilla/prettyplease-cli/workflows/CI/badge.svg)](
https://github.com/danielhenrymantilla/prettyplease-cli/actions)

<!-- Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template -->

Unofficial and basic CLI wrapper around the [`::prettyplease`] library.

[`::prettyplease`]: https://docs.rs/prettyplease

This can be used as a poorman's substitute of `rustfmt`, **mainly for generated
code**.

#### ⚠️ Warning: comments are lost ⚠️

Indeed, [`::prettyplease`] only operates off a [`::syn::File`] (by design). And
a [`::syn::File`] is an AST[^1] parsed representation of a file of source code.
Such AST _does not include comments_.

[^1]: Abstract Syntax Tree

[`::syn::File`]: https://docs.rs/syn/^1.0.0/syn/struct.File.html

This means that piping the contents of a source file into
`::syn::parse_file()` and then into `::prettyplease::unparse()` results in
**loss of comments**; and this is exactly what this CLI tool does.

  - Note, however, that _doc-comments_ are preserved, since those are part of
    Rust's AST.

### Installation

```bash
cargo install prettyplease-cli
## You may add `--no-default-features` to speed up the compilation
## if you are not interested in the `--check` mode showing pretty line diffs.
```

### Usage

```console
Unofficial CLI wrapper around `::prettyplease::unparse` to format files and trim comments.

This modifies them IN PLACE, unless the `--check` flag is passed.

USAGE:
    prettyplease-fmt [OPTION] <files…>

OPTIONS:
        --check     Runs in 'check' mode. Exits with 0 if input is formatted
                    correctly. Exits with 1 if formatting is required, printing
                    a diff unless the `check-shows-diffs` Cargo feature were
                    disabled at compile-time.
    -h, --help      Shows this very message.

See https://crates.io/crates/prettyplease for more info about the formatting itself.
```

#### Missing features / FIXME

  - [ ] Preserve comments, somehow;

  - [ ] Accept files and/or globs for directory traversal;

  - [ ] Handle files in parallel.
