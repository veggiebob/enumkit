# enumkit_derive

This crate provides proc macros for use with [`enumkit`](https://crates.io/crates/enumkit):

- `#[derive(EnumValues)]` – Adds a `values()` method that returns all unit enum variants.
- `#[derive(EnumMapping)]` – Generates a struct mapping enum variants to values in a static array.

These macros only work with enums composed **entirely of unit variants** (i.e., variants with no associated data).

---

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
enumkit_derive = "<version>"
````

Then derive the macros on your enum:

```rust
use enum_tools_derive::{EnumValues, EnumMapping};

#[derive(EnumValues, EnumMapping)]
enum Mode {
    Fast,
    Slow,
    Sleep,
}
```

See enumkit for the trait definitions and runtime behavior.

---

## Notes

- These macros panic at compile time if used on non-unit enums.
- Serialization support via `serde` is opt-in via feature flags in `enumkit`.

---

## License

MIT or Apache-2.0
