# Navigating between pages
In Next.js a page is a React Component exported from a file in the `pages`
directory, its associated with a route based on its file name. 

In Next.js, we can use the `Link` component to between pages in our application.
It allows us to do client side navigation and accepts `props` that gives us
better control over the navigation behavior.

> For external links `a` tag should be used instead.

## Client side navigation
`Link` enables client side navigation between two pages in the same Next.js app.

**Client side navigation** means that the transition between pages happens using
JavaScript, which is faster than the default navigation done by the browser.

## Code splitting and prefetching
Next.js does code spliting automatically, each page only loads what's necessary
for a given pâge. This has the added benefit of isolating pages, if a certain 
page throws an error, the rest of the application would still work. 

In production builds, whenever `Link` components appear in the browser's 
viewport Next.js automatically prefetches the code for the linked page in the
background. 
