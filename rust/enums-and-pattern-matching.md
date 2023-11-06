# Enums and pattern matching

- Enums allow us to define a type by enumerating its possible variants.

## Defining an enum

- Enums gives us a way of saying that a value is one of a possible set of
  values.
```rust
enum IpAddrKind {
    V4,
    V6,
}
```

## Enum values

- Instances of enums can be created like so:
```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

- Data can be given to each variant:
```rust
enum IpAddr {
    V4(String),
    V6(String),
}
```

> The name of each enum variant defined with data, also becomes a function that
  constructs an instance of the enum.

- Just like with structs we can define methods for enums using `impl`.

## The `Option` enum and its andvantages over null values

- The `Option` enum defined in std lib, is used for the scenario in which a
  value could be something or it could be nothing.
- Rust doesn't have the null feature that other programming languages have. 

