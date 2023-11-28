# Iteration

- Leptos supports two different patterns for iterating over items: 
    - For static views: `vec<_>`.
    - For dynamic lists: `<For />`.

## Static views with Vec<_>

- We can isert any `Vec<IV> where IV: IntoView` into a view. 
- Leptos provides a `.collect_view()` helper function that allows us to 
  collect any iterator of `T: IntoView` into `Vec<View>`.
- Dynamic items can be rendered as part of a static list.

## Dynamic Rendering with the `<For />` component

- `<For />` is a keyed dynamic list, it takes three props:
    - `each`: a function that returns the items `T` to be iterated over.
    - `key`: a key function that takes `&T` and returns a stable, unique key or
      ID.
    - `children`: renders each `T` into a view.

## Iterating over more complex data with `<For />`

- `RwSignal<_>` is a read and write signal, which combines the getter and
  setter in one object.
- Leptos provides a primitive called `create_memo`, it creates a derived
  computation that only triggers a reactive update when its value has changed.
    - This allows us to create reactive values for subfields of larger data
      structures, without having to wrap fields in signals.

