# Anatomy of a Rust program

- The `main` function is the entry point of rust programs.
- `rustfmt` is formatting tool, that auto formats rust code.
- `!` is used to call _macros_. Macros do not follow the same rules as 
   functions.
- Rust is an _ahead of time_ compiled language.
- Rust has a set of items, defined in standard lib that are brought in the
  scope of every program. This is called the _prelude_.
- `let` creates variables.
- Variables are immutable by default.
- To make a variable mutable we can make use of `mut`.
```Rust
let mut guess = String::new();

io::stdin()
    .read_line(&mut guess);
    .expect("Failed to read line");
```
- _Shadowing_ allows for the re-use of variable names. Often used to
  convert a type into another type.
- Associated functions are functions that are associate with a type.
- Match expressions are composed of arms. An arm consists of a pattern
  to match against the value that was passed in.
```Rust
match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small");
        Ordering::Greater => println!("Too big");
        Ordering::EEqual => println!("You win");
    }
```

# Build system and package managers

- Cargo is rust's build system and package manager.
- A cargo project is created with `create new <project_name>`.
- `cargo new` creates a src folder and a `Cargo.toml` for configurating cargo. 
  If a cargo project is not created in a version control repo. It will create one.
- Packages are refferd to as _creates_.
- `cargo build` builds a project. The default build is debug. passing it the
  `--release` flag, `cargo` creates a release build.
- `Cargo.lock` keeps track of the versions of the dependencies.
- `cargo run` compiles and runs a project.
- `cargo check` check the code to see if it compiles but it does not create a
  binary.


