# Ownership

- Ownership is the set of rules that govern how rust manages memory.
- In rust memory is managed through a system of ownership with a set of rules
  that the compiler checks. If any of the rules are violated the program won't
  compile.
- In systems programming languages like rust, whether a value is on the stack
  or the heap affects how the language behaves.
- The stack and the heap are parts of memory avaible to programs at runtime.
  But they differ in some ways.
    - The stack is structered in _lifo_ - Last in, first out.
    - All data added to the stack must have a fixed size.
    - Data with an unknown size at compile time or a size that changes must be
      stored at the heap.
    - When allocating memory on the heap, the computer finds a spot that is big
      enough and returns a pointer to that location.
    - Pushing to the stack is faster then allocating memory.
    - Accessing data on the heap is slower than accessing data on the stack,
      because we must follow the pointer to get there.

> Pushing values to the stack is not considered allocating.

- Ownership addresses the issues of having to keep track of what parts of the
  code are using what data on the heap, minimizing the amounts of duplicate
  data on the heap and cleaning up unused data on the heap.
- The ownership rules are as follows:
  - Each value has an owner.
  - There can only be one owner at a time.
  - When the owner gets out of scope, the value will be dropped.
- When a variable goes out of scope Rust calls a function called `drop`, this 
  function is called automatically at the closing bracket.

## Variables and data interacting with move

- When a shallow copy is made, rust invalidates the first variable.
  Instead of being called shallow copy in rust it's known as a _move_.
- Rust will never automatically create deep copies of data, therefore any
  automatic copying can be assumed to be inexpensive in terms of runtime
  performance.

## Variables and data interacting with `clone`

- To deeply copy the heap data, the `clone` method can be used.

## Stack only data: `Copy`

- For types that are stored on the stack rust has a special annotation, the
  `Copy` trait. Types that implement this trait, variables that use them do
  not move, but copied, making them still valid after assignment to another
  variable.
- Rust won't allow to annotate a type with `Copy` if the type, or any of its
  parts, has implmented the `Drop` trait.
- Scalar values can implement the `Copy` trait.

## Ownership and functions

- When passing a value to a function, it can either be moved or coppied.

## Return values and scope

- Returning values can also transfer ownership.
- Rust allows for returning multiple values using a tuple.
- Rust's feature for using a value without transferring ownership is called 
  _references_.
- A reference is similar to a pointer in that its an address in memory, 
  however a reference will always point to a valid value for the life of that
  reference.
- `&` represents a refference. The opppiste of a reference is called 
  dereferencing and its represented by `*`.
- Creating a reference is called _borrowing_. 
- By default values of references are immutable.

## Mutable references 

- For a reference to be mutable:
    - The varibale referenced must be declared using `mut`.
    - A mutable reference is created using `&mut`.
    - `&mut` needs to be added to the function signature.
- There can only be one mutable reference at once for a given value.
    - This restrictions prevents data races at compile time
    - A data race is similar to a race condition, it happens when:
        - Two or more pointers acess the same data at the same time.
        - At least one of the pointers is being used to write on the data.
        - There's no mechanism being used to synchronize access to the data.
- A similar rule is enfored for combining mutable and immutable references.

> A references scope starts from where it is introduced and continues through
  the last time its used.

## The slice type

- Slices lets us reference a contiguous sequence of elements in a collection.
- A slice is a kind of reference, therefore it does not have ownership.

## String slices

- A string slice is a reference to part of a `String`.
```Rust
let s = String::from("hello, world");

let hello = &s[0..5];
let world = &s[6..11];
```

- String slices are created by specifing a range within brackets 
  `[starting_index..ending_index]`.
- Under the hood, the slica data structutre stores the starting postion and the
  length of the slice - `ending_index - starting_index`.
- With range syntax `..`, if starting from index 0, the starting index can be
  omitted similarly if we want to include the last byte of the string, the
  ending index can also be omitted. Both values can be omitted to take a slice
  of the entire string.

> String slice range indices must occur at valid UTF-8 character boundaries.
  Attempting to create a string slice in the midle of a multibyte character,
  the program will exit with an error.

- The type that defines a string slice is `&str`.
- String literals are of type `&str`, this explains why ther are immutable;
  `&str` is an immutable reference.
