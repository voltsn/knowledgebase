# View: dynamic classes, styles and attributes

## Dynamic classes

- Css classes can be updated dynamically using the `class` syntax:
  `class:red=move || count() % 2 == 1`.
- `class:` attributes take:
    - After the colon the class name.
    - A value, which can be a `bool`or a function that returns a `bool`.
- If the value is a function that access a reactive value, the class will be
  reactively updated when the signal changes.
- Some css class names cannot be direclty parsed by the `view` macro.
  Especially if they include a mix of dashes and numbers or other characters.
  In that case, we can use the tuple syntax `class=("name", value)`.

## Dynamic styles

- Individual css properties can be directly updated with the `style` syntax.
```rust
let (x, set_x) = create_signal(0);
let (y, set_y) = create_signal(0);

view! {
    <div
        style="position: absolute"
        style:left=move || format!{"{}px", x() + 100)
        style:top=move || format!("{}px", y() + 100)
        style:background-color=move || format!("rgb({}, {}, 100)", x(), y())
        style("--columns", x)
    >
        "Moves when coordinates change"
    </div>
}
```

## Dynamic attributes

- Rules applied to the `style syntax` also apply to attributes.

## Derived signals

- Derived signals let us create reactive computed values that can be usd in
  multiple places with minimal overhead.

> Using derived signals likes means that calculation runs once per signal
  change and once per place where we access the derived signal.

```rust
let double_count = move || count() * 2;

<progess
    max="50"
    value=double_count
/>
<p>
    "Double count: "
    {double_count}
</p>
```

## Injecting raw html

- The `view` macro provides support setting the html contents of any element
  using `inner_html`. *It does not escape the porived html*.
