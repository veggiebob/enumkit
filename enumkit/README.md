# enumkit

`enumkit` provides traits and utilities for working with enums that consist only of unit variants. It works hand-in-hand with its companion crate, [`enumkit_derive`](https://crates.io/crates/enumkit_derive), which provides `#[derive(EnumValues)]` and `#[derive(EnumMapping)]` proc macros.

---

## Features

- `EnumValues`: Automatically enumerate all variants of a unit enum.
- `EnumMapping<T>`: Efficiently associate values with enum variants using a static-sized array.
- `serde` (optional): Enables `Serialize` and `Deserialize` derives for mapping types.

---

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
enumkit = "<version>"
```

Then derive traits for your enum:

```rust
use enum_tools::{EnumValues, EnumMapping};

#[derive(EnumValues, EnumMapping, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Start,
    Middle,
    End,
}

fn main() {
    // EnumValues usage
    for variant in State::values() {
        println!("{:?}", variant);
    }

    // EnumMapping usage
    let mut map = StateMapping::new(|s| format!("{:?}", s));
    assert_eq!(map.get(State::Middle), "Middle");
}
```

---

## Feature Flags

- `serde`: Enables `Serialize` and `Deserialize` for mapping structs.

---

## License

MIT or Apache-2.0
