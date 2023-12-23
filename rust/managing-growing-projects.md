# Managing growing projects with packages, crates and modules

- As a project grows we should organize our code by splitting it into multiple
  modules and then multiple files.
- A package can contain multiple binary crates and optionally one library
  crate.
- The module system, includes:
    - *Packages*: cargo feature that lets us build, test and share crates.
    - *Crates*: a tree of modules that produces a library or executable.
    - *Modules* and *use*: let us control the organisation, scope and privacy
      of paths.
    - *Paths*: a way of naming an item, suchs a struct, function or a module.

## Packages and crates

- A crate is the smallest amount of code that the Rust compiler considers at
  at a time.
- Crates can contain modules and the modules can be defined in other files that
  get compiled with the crate.
- A crate can be one of two things, a *binary* or a *library*.
    - A binary crate is a program that we can compile to an executable, that
      we can run.
        - Each binary crate must have a `main` function that defines what
          happens when the program is run.
    - Library crates don't have a `main` function and they do not compile to
      an executable.
        - The define functionality intended to be shared with multiple
          projects.
- *The crate root* is a source file that the rust compiler starts from and
  makes up the root module of our crate.
- A *package* is a bundle of one or more crates that provide a set of
  functionality.
    - A package contains a `Cargo.toml` file that describes how to buid those
      crates. 
    - A package can contain as many crates as we want but it can contain at
      one library crate.
    - A package must contain at least one crate, whether that's a library or
      binary crate.
- Cargo follows a convention that `src/main.rs` is the crate root of a binary
  crate with the same name as the package.
- In a package directory containing `src/lib.rs`, cargo will treat it as the
  crate root.
- Cargo passes the crate root files to `rustc` to build the library or 
  binary.

## Defining modules to control scope and privacy

### Grouping related code in modules

- Modules let us organize code within a crate for readability and easy reuse.
- They also allows to control the privacy of items. By default code within a
  module is private.
- Modules are defined with the `mod` keyword followed by the name of the
  module.
```Rust
mod front_of_house {
    mod hosting {
        ...
    }
}
```
- Modules can contain other modules, sturcts, enums, constants, traits and
  functions.
- `src/lib.rs` and `src/main.rs` are called crate roots, because the contents
  of either of these two files form a module named `crate` at the root of the
  crate's module structure known as the module tree.
```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

### Paths for referring to an item in the module tree

- A path shows rust where to find an item in a module tree.
- A path can take two forms:
    - An _absolute path_: the full path starting from a crate root;
      for code from an external crate, the absolute path begins with the crate
      name, and for the current crate, it starts with the literal `crate`.
    - A _relative path_: starts from the current moduse and uses `self`,
      `super` or an identifier in the current module.
- Both forms are followed by one or more identifiers seperated by `::`.


### Starting relative paths with `super`

- Relative paths that begin in the parent module can be constructed by using
  `super` at the start of the path.
- `super` allow us to reference an item that is in the parent module.

## Bringing paths into scope with the `use` keyword

- We can create a shortcut to a path with the `use` keyword once, and then
  use the shorter name everywhere else in the scope.
- `use` only creetes the shortcut for the scope in which the use occurs.

### Creating idiomatic `use` paths

- The idiomatic way of defining shortcuts to paths with `use` is:
  `use crate::front_of_house::hosting`. Bringing modules into scope 
  this way means that we have to specify the parent module when calling
  a function. This makes it clear where the function come from.
- When importing structs, enums and other items with `use`, it's idiomatic
  to specify the full path.

### Providing new names with the `as` keyword

- After `use` we can specify `as` and a new local name/alias for the type.
  `use std::io::Result as IoResult;`

### Using nested paths to clean up large `use` lists

- We can use nested paths to bring items into scope in one line:
`use std::{cmp::Ordering, io};`.
- We can combine two `use` statements that share a subpath:
```Rust
use std::io;
use std::io::Write;

use std::io{Self, Write};
```
### The glob operator

- We can bring all public items defined in a path into scope, by specifying
  the path followed by `*`.

> The global operator can make it harder to tell what names are in scope and
  where a name used in our program was defined.
