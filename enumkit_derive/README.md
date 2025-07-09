# Enumkit

There are currently 2 derive macros: `EnumValues` and `EnumMapping`.
Both are only intended to be used for enums which are exclusively unit variants, meaning they look something
like this:

```rust
enum X {
    A,
    B,
    C
}
```

`EnumMapping` is used to provide iterators and constructors for a fixed-size static mapping from the variants
to the user's type. If the name of your enum is `Enum`, it expands with these definitions:
- `EnumMapping` - the data structure that holds the mapping. This has a similar API to `HashMap<Enum, T>`
- `EnumMappingIntoIter`
- `EnumMappingIter`