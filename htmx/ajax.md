# AJAX

-  The core of htmx is a set of attributes that allows us to issue AJAX
   requests directly from html:

   | Attribute | Description      |
   ------------|------------------|
   | hx-get    | `GET` request    |
   | hx-post   | `POST` request   |
   | hx-put    | `PUT` request    |
   | hx-patch  | `PATCH` request  |
   | hx-delete | `DELETE` request |

    - Each of these attributes takes a URL to issue an AJAX request to. The
      element will issue the request when its triggered.
      ```html
      <div hx-put="/messages">
        Put to messages
      </div>

      <!-- 
        When a user clicks on this div, a PUT request is issued to the
        /message url.
      -->
      ```
## Triggers
    
- By default AJAX requests are triggered by the "natural" event of an element,
  such as `subimt` for forms or `change` for inputs.
    - This behaviour can be modified by using the `hx-trigger` attribute to
      specify which event will cause the request.
      ```html
      <div hx-post="/mouse_entered" hx-trigger="mouseenter">
        [Here mouse, mouse!]
      </div>
      ```
### Trigger modifiers

- A trigger can have a few modifiers that change its behavior.
```html
<div hx-post="/mouse_enetered" hx-trigger="mouseenter once">
    [Here mouse, mouse!]
</div>
```
- Here is list of available modifiers:
    - `changed' - issue a request if the value of the element has changed.
    - `delay:<time_interval>`- wait the given amount of time before issuing
       the request. If the event triggers again, the countdown is rest.
    - `throttle:<time_interval> - wait the given amount of time before
       issuing the request. In contrast to `delay`, if a new event occurs
       before the time limit is hit, the new event is discarded.
    - `from:<css_selector>` - listen for the event on a different element.
       this can be used for keyboard shortcuts.
```html
<input type="text" name="q"
    hx-get="/trigger_delay"
    hx-trigger="keyup changed delay:500ms"
    hx-target="#search-results"
    placeholder="Search..."
>
<div id="search-results"></div>
```
### Trigger filtering

- Filters may be applied to triggers by using square brackets after the
  event name, enclosing a js expression that will be evaluated. If the
  expression is evaluated to `true` the event will trigger.
```html
<div hx-get="/clicked" hx-trigger="click[ctrlKey]">
    Control click me
</div>
```

### Special events

- HTMX provides a few special events for use with `hx-trigger`:
    - `load` - fires once when the element is first loaded.
    - `revealed` - fires once when an element first scroll into the viewport.
    - `intersect` - fires once when an element first intersects the viewport.
      It supports two additional options:
        - `root:<selector> - a css selector of the root element for 
           intersection.
        - `threshold:<float> - a floating point number between `0.0` and `1.0`,
           indicating what amount of intersection to fire the event on.

> For advanced use cases, custom events can be used to trigger requests.
