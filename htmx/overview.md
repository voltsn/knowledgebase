# Overview

- HTMX is a library that allows us to access to modern browser features
  directly from html, instead of using javascript.
- HTMX extends and generalizes the core idea of html as a hypertext, opening
  more possibilities directly within the langugae such as:
    - Any element can issue an http request.
    - Any event can trigger requests.
    - Any http verb can be used.
    - Any element can be the target for update by the request.

> When using htmx, the server typically respond with html not json.
  This keeps us firmly alligned with the original web programming model.

Example of what htmx looks like:
```html
<button hx-post="/clicked"
        hx-trigger="click"
        hx-target="#parent-div"
        hx-swap="outerHTML"
>
    Click me!
</button>
```

## Installing

- To use htmx we can:
    - Add a `script` tag to the document head:
    ```html
    <script 
        src="https://unpkg.com/htmx.org@1.9.7" 
        integrity="sha384-EAzY246d6BpbWR7sQ8+WEm40J8c3dHFsqC58IgPlh4kMbRRI6P6WA+LA/qGAyAu8" 
        crossorigin="anonymous"
    >
    </script>
    ```
    - Download a copy and add it to our project, extension can also be added
      this way.
    - Using npm `npm install htmx.org`.

