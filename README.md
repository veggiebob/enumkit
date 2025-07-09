# enumkit workspace

This repository contains the `enumkit` ecosystem. See README files in each crate for details.

---

## Crates

### [`enumkit`](./enumkit)

A runtime utility crate that provides traits such as:

- `EnumValues`: Enumerate all variants of a unit enum.
- `EnumMapping<T>`: Associate values with enum variants using a fixed-size array.

Optional `serde` support can be enabled via a feature flag.

➡️ See [`enumkit/README.md`](./enumkit/README.md) for details.

---

### [`enumkit-derive`](./enumkit-derive)

A proc macro crate providing:

- `#[derive(EnumValues)]`
- `#[derive(EnumMapping)]`

These derive the `EnumValues` and `EnumMapping` traits for enums that consist entirely of unit variants (i.e., variants with no associated data).
They are also exposed through the `enumkit` crate.

➡️ See [`enumkit-derive/README.md`](enumkit-derive/README.md) for usage instructions.

---

## Getting Started

To use the `enumkit` crate, add it to your `Cargo.toml`:

```toml
[dependencies]
enumkit = "<version>"
```

You can then use the provided derive macros and traits in your code:

```rust
use enumkit::{EnumValues, EnumMapping};

#[derive(EnumValues, EnumMapping)]
enum Example {
    A,
    B,
    C,
}
```

---

## Workspace Structure

```
enumkit/          # Core trait implementations and utilities
enumkit-derive/   # Procedural macros for EnumValues and EnumMapping
```

The crates are published independently but developed together for cohesion and convenience.

---

## License

MIT or Apache-2.0
