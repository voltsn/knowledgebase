# Using structs to structure related data

- A struct is a custom data type that lets us package together and name multiple
  related values that make up a meaningful group.
- Structs and enums are the building blocks for creating new types to take full
  advantage of rust's compile time type checking.

## Defining and instanting structs

- A struct is defined like so:
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

- In order to use a struct we create an instance of said struct.
- Values can be extracted or modified if its a mutable instance, using the `.` 
  notation.

## Struct update syntax

- The _field init shorthand_ syntax allows to use the names of the parameters
  of a function inside a struct under the condition that the fields of the 
  struct are indentical to the parameters name.
```rust
fn build_user(emial: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

- Its possible to create instance from other instances using the _struct update
  syntax_.
```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
}
```

- The struct update syntax should appear last. This tells rust that any
  remaining fields should comme from the 'other' instance.
- When using the update syntax, if the data contains values allocated in the 
  heap, the data is moved making the previous instance no longer valid.

## Tupple structs without named fields

- Tupple structs don't have names associated with their fields, they just have
  the types of the associated fields.
- Used when:
    - Naming each field is verbose or redundant.
    - Giving a tuple a name.
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
- Struct tuples can be destructerd to their own indivdual pieces.
- The dot notation followed by an index can be used to access a value.

## Structs without any fields

- Structs without any fields are called **unit-like structs**.
- They similarly to `()`.
- Useful when:
    - We need to implement a trait on some type but don't have any data
      to store in the type itself.
```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

## Method syntax

- The first parameter of a method is always `self`, which represents the 
  instance as a parameter.
- To define methods for a struct we start by an `impl` block for the struct.
- Everything inside the `impl` block will be associated with a given struct.
- `&self` is short for `self: &Self`.
- The type `Self` is an alias for the type that the `impl` block is for.

> A method can have the same name as one the struct's fields.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width($self) -> bool {
        self.width > 0
    }
}
```

- All functions defined within an `impl` block are called associated
  functions.
- Associated functions can be created without passing `&self` as a parameter.
    - They are often used as constructors, these are often called `new`.
    - They are called using the `::` syntax. This syntax is used also for 
      namespaces.
```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```
