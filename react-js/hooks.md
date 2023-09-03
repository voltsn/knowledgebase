# React Hooks
In React any functio starting with _use_, is called a Hook.
Hooks are special functions that are only available while React
is rendering. They allows us to _hook into_ different React
features.

> Hooks can only be called at the top level of a component or a
  custom hook. They cannot be called inside conditions, loops
  or other nested functions. It's helpful to think of hooks as 
  unconditional declarions about a component's needs.

## `useRef`
`useRef` is a hook that lets us reference a value that's not needed for
rendering.
