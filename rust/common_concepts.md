# Constanst

- Constants like immutable variables, cannot be changed. However there are 
  differences between them.
  - `mut` cannot be used with constants.
  - Constants are always immutable.
  - Constants are declared with `const` and their type must be annotated.
  - Constants can be declared in any scope.
  - Constants can be declared only in constant expression:
    `const THREE_HOURS_IN_SECONDS = 60 * 60 * 3`

# Shadowing

- When declaring a varibale with the same name as a previous one, its called
  shadowing.
- The second value _overshadows_ the first, taking use of the varible name for
  itself until it self is overshadowed or the scope ends.
- Variables are overshadowed when using their name and the `let` keyword.
- Changing the value of an immutable variable without let will cause a compile
  time error.
- By overshadowing a variable we can apply transformations on a value and keep
  the variable immutable after the transformation has taken place.
- When using the let keyword we can change the type of associated with a
  variable name.

# Data types

- Rust is a statically type language.
- The compiler can usally infer the types of variables. However in cases in
  which multiple types are possible, the type must be specified.

## Scalar types

- They represent a single value.
- Ther are four primary scalar types:
  - Integers
  - Floats
  - Booleans
  - Characters

## Integer types

- There are multiple integer types in rust

|  Length | signed | unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| isize   | isize  | usize    |

- Signed and unsigned define whether or not a number can be negative.
- ==Signed numbers are store using the two's compelemnet representation==.
- Each signed variant can store numbers from -(2^n-1) to 2^n-1 - 1. Where n is
  the number of bits.
- `isize` and `usize` depend on the architecture of the computer the program
  is running on. 32-bits on a 32-bit integer, 64-bit on a 64-bit integer.
- Number litterals can be represented in multiple forms. 
- We can use `_` as visual separtion to make numbers easier to read.

| Number litteral | Example     |
|-----------------|-------------|
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | ob1111_0000 |
| Byte (u8 only)  | b'A'        |

- The default integer type is i32.
