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

- Rust doesn't have the null feature that other programming languages have. 
- The `Option<T>` enum defined in std-lib, encodes the concept of a value being,
  being present or absent.
- The `<T>` syntax, is a generic type parameter.  

## The `match` control flow contstuct

- The `match` construct allows you to compare a value angainst a series of
  patterns and then execute code based on which pattern matches.
    - Patterns can be: literal values, variable names, wildcards...
- The power of `match` comes from the expressiveness of the patterns and the 
  fact the compiler confirms that all possible cases are handled.
```Rust
enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
- `match` arms can bind to the parts of the values that match the pattern. 
- `match` can be used with `Option<T>`.
- `match` is exhaustive.
- Catch-all patterns allow us to specify a default action for some values.
- `_` is a placeholder, it's a special pattern that matches any value and does
  not bind to that value.

## Concise control flow with `if let`

- The `if let` syntax lets us combine `if` and `let` into a less verbose way
  way to handle values that match one patern while ignoring the rest. 
```Rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```
- It takes a pattern and an expression separated by an equal sign. 
- The code isn't run if the value doesn't match the pattern.
- `if let` is syntactic sugar for a `match` that runs code when the values 
  matches one pattern and ignores all the other values.
- An `else` block can be inlcuded in an `if let`. It would be the same as using
  `_` case in the `match` expression.
