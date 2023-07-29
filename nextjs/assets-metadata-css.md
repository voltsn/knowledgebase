# Assets
We can serve static assets, such as images under the `public` directory. The
files inside `public` can be referenced from the root of the application similar
to `pages`. 

For images Next.js provides the `Image` component to handle:
- Images being responsive on different screen sizes.
- Optimizing images.
- Loading images when they enter the viewport.
- ...

`next/image` is an extension of the `<img>` element.

```js
import Image from 'next/image'
```

Next.js has support for image optimization by default, it works with any image
source.

Images are optimized on demand, per user request. They are lazyloaded by defaut
and are always rendered in such a way as to avoid cumulative layout shift, a
core web vital that google is going to use in search ranking.

# Metadata
the metadata of a page can be modified by using the `Head` component.

```js
import Head from 'next/head'
```

# Third party scripts
`next/script` is an extension of the `<script>` element and optimizes when 
additional scripts are fetched and executed.

```js
import Script from 'next/script'
```

- `strategy` controls when third party script should load.
- `onLoad` is used to run any JavaScript code immediately after the script has
   finished loading. 


