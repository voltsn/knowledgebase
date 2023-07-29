# How it works

## Dev and production environments
Environemts can be thought of as the context in which our code is running

During development, the application is run and built on our local machine. 
Production is the environment in which our application is deployed and consumed
by users. 

Next.js provides features for both stages of an application.

Since each environment has different considerations and goals, a lot has to be
done to move an application from development. For example the application code
needs to be:
- Compiled
- Bundled
- Minified 
- Code split

## The Next.js compiler
Next.js handles the code transformation and underlying infrastructure needed to
make it easier for our application to go to production. 

Next.js has a compiler written in Rust. In Next.js compilation happens during 
the development stage, and the build step to prepare you application for 
production. 

## Minifying
In Next.js, Javascript and CSS are automatically minified for production. 
Minification is the process of removing unnecessary code formatting and comments
without changing the code's functionality, with the goal of improving an 
applications performance by decreasing file sizes. 

## Bundling
Bundling is the process of resolving the web of dependencies and merging or 
packaging the files or modules into optimized bundles for the browser, with the
goal of reducing the number of requests for files when a user visits a web page.

## Code spliting
Code splitting is the process of splitting the application's bundle into smaller
chunks required by each entry point, with the purpose of improving the load time
by only loading the requried to run that page.

Next.js has built in support for code splitting. Each file inside the `pages/`
directory will be automatically code split into its own JavaScript bundle during
the build step. 

- Any code shared between pages is also split into another bundle to avoid re-
  downloading the same code on further navigation.
- After the initial page load, Next.js can start pre-loading the code of other
  pages users are likely to navigate to.
- Dynamic imports are another way to manually split what code is initially
  loaded.

## Rendering
Rendering is the process of converting React code into the HTML representation
of our UI. 

### Pre-rendering
Server-Side rendering and static site generation are also reffered to as 
pre-rendering because the fetching of external data and transformation of React
components into HTML happens before the result is sent to the client. Next.js
pre-renders every page by default. 

### Server Side rendering
**Hydration** is the process in which on the client, the HTML is used to show a
fast non-interactive page. React uses JSON data and JavaScript instructions to 
make components interactive.

In Next.js, we can opt to server side render pages by using `getServerSideProps`

### Static site generation
Unlike server side rendering there is no server running at runtime. Static site
generation generates content onces, at build time when an applcation is deployed.
The HTML is stored in a CDN and re-used for each request. 

In Next.js its possible to statically generate pages by using `getStaticProps`.

