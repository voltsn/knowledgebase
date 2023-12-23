# Forms and inputs

- There are two patterns for interacting with inputs that are similar to
  frameworks such as react and solidJS:
    - Controlled inputs
    - Uncontrolled inputs

## Controlled inputs

- In a controlled input, the framework controls the state of the input element.
- On every input event, a local signal is updated that holds the current state,
  which in turn updates the `value` prop of the input.
- The input event fires on almost every change to the element.
- The change event fires more or less when the input is unfocused.
- The `value` attribute only sets the initial value of the input. 
- For controlled inputs is best to use `prop:` syntax.

## Uncontrolled inputs

- In an uncontrolled input the browser controls the state of the input
  element.
- A `NodeRef` is used to access the input once when we want to get its value.
    - It's a kind of reactive smart pointter.
    - It can be used to access the underlying DOM node.
    - Its value is set when the element is rendered.
