# Components and props

- Similarly to frontend frameworks leptos components accept properties.
- The only way to tell the interface to resopond to change is to pass it a 
  signal type.

## Optional props

- Optional props are annotated with `#prop(optional)]` attribute.
```rust
#[component]
fn progressB(
    #prop(optional)]
    max: u16,
    progress: ReadSignal<i32>
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}
```

## Default props

- Other than `Default::default()` default values can be defined with
  `#[prop(default = ...)]` attribute.

## Into props

- The `#[prop(into)]` attribute automatically calls `.into()`.
- The `Signal` type is an enum that represents any kind of readable reactive
  signal.
- The `MaybeSignal` types allows to take either a static or reactive value.

## Optional generic props

- Optional generic props cannot be specified for components. 

## Documenting components

- To document a component and its props, we can add doc comments on the
  component function.


## `#[component(transparent)]`

- This macro allows for a component to return something other than 
  `impl IntoView`.
- It is used mostly in two situations:
    - Creating wrappers around `<Supense />` or `<Transition />`. Both of them
      return a transparent suspense structure.
    - Refactoring `<Route />` definition for `leptos_router` out into separate
      components, because `<Route />` is a transarent component that retuns a 
      `RouteDefinition` struct instead than view.
