# App router

- Starting version 13 **App router** got introduced, It was built on top of
  react server components.
- App router works in a directory called `app`.
- `app` works allong the `pages` directory allowing for partial adoption.
  - It takes priority over the `pages` directory. 
- By default components inside `app` are server components. Client components
  can be used aswell.
- Routes are difined by folders.
- Files are used to create the ui.
- Each folder in a route represents a path segement

`dashbord/settings`

- Nested routes can created by nesting folders. 
- Only contents returned by `pages.js` or `route.js` are publicly accessible.
