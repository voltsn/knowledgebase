# A basic component

- Components represent a section of the DOM, with self-contained, defined 
  behavior. Unlike html elements, they are in PascalCase.
 
## The component signature

- All component definitions, begin with the `#[component]` macro.
    - It annotates a function so that it can be used as a component in leptos
      applications.
- All components are functions with the following charecteristics: 
    - They take zero or more arguments of any type.
    - They return `impl IntoView` which is an opaque type that includes
      anything that can be returned from a leptos `view`.

> Component function arguments are gathered together into a single props struct 
  which is built by the view `macro` as needed.

## The component body

- The body of a component function is a set-up function that runs once.
    - It is not a render function that runs multiple times.
    - It is typically used to:
        - Create reactive variables.
        - Define any side effects that run in response to those variables
          changing.
        - Describe user interfaces.
- `create_signal`, creates a signal, the basic unit of reactive change and
  state management in leptos. It returns a getter and setter tuple.

`let (count, set_count) = create_signal(0);`

- The current value can be accessed using `count.get()` or on nightly 
  `(count())`, to set the current value we call `set_count.set()` or 
  `set_count()`.

## The view

- User interfaces are defined usin a jsx like format viw the `view` macro.
```rust
view! {
    <button
        on:click=move |_| {
                set_count(3);
        }
    >
        "Click me:"
        {count()}
    </button>
}
```
- Passing a function into a view tells leptos that its something that might 
  change.
- When a reactive value changes the closure reruns and only the html element
  containing the value will be updated. Nothing else is touched.
- Only functiobns are reactive.

