[![License](https://img.shields.io/crates/l/serde-format.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/serde-format.svg)](https://crates.io/crates/serde-format)
[![Docs.rs](https://docs.rs/serde-format/badge.svg)](https://docs.rs/serde-format)

<!-- cargo-sync-readme start -->

# serde-format

A tiny trait to format a serializable struct using custom placeholders.

## Goals

- Be as lightweight as possible
- Have no dependencies other than [serde] and [serde_json]

## Non-goals

- Prioritize performance
- Support any syntax beyond variable substitution

## Usage

```rust
use serde::Serialize;
use serde_format::Format;

#[derive(Serialize)]
struct Foo {
    name: String
}

impl Format for Foo {}

let foo = Foo { name: "Bar".into() };
assert_eq!(foo.format("Hey, {{name}}!"), "Hey, Bar!");
```

<!-- cargo-sync-readme end -->

## Contributing

We appreciate all kinds of contributions, thank you!


### Note on README

Most of the readme is automatically copied from the crate documentation by [cargo-sync-readme][].
This way the readme is always in sync with the docs and examples are tested.

So if you find a part of the readme you'd like to change between `<!-- cargo-sync-readme start -->`
and `<!-- cargo-sync-readme end -->` markers, don't edit `README.md` directly, but rather change
the documentation on top of `src/lib.rs` and then synchronize the readme with:
```bash
cargo sync-readme
```
(make sure the cargo command is installed):
```bash
cargo install cargo-sync-readme
```

If you have [rusty-hook] installed the changes will apply automatically on commit.


## License

This project is licensed under the [MIT license](LICENSE).

[cargo-sync-readme]: https://github.com/phaazon/cargo-sync-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
