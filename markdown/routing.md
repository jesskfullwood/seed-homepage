# Routing
Seed includes routing: You can trigger state changes that update the address bar,
 and can be nagivated to/from using forward and back buttons. This works for landing-page
routing as well, provided your server is configured to support. For an example of routes in use,
see the [homepage](https://github.com/David-OConnor/seed/tree/master/examples/homepage) or
[todomvc](https://github.com/David-OConnor/seed/tree/master/examples/todomvc) examples.
  
As an example, let's say our site has three pages:
a home page, a guide, and a changelog, accessible by `http://seed-rs.org/`, `http://seed-rs.org/guide`,
and `http://seed-rs.org/changelog` respectively. We describe the page by a `page`
field in our model, which is an integer: 0 for homepage, 1 for guide, or 2 for changelog.
(An enum would work as well). 

To set up the initial routing, we pass a `HashMap<String, Msg>` describing the possible routings
as the last parameter of [Seed::run](https://docs.rs/seed/0.1.10/seed/fn.run.html). We can
create it using the `routes!` macro, which is for convenience: Rust doesn't include a
HashMap literal syntax, and this macro automatically converts the keys to Strings, eg
in the case of the &strs we use in the example below:
```rust
#[wasm_bindgen]
pub fn render() {
    let routes = routes!{
        "guide" => Msg::RoutePage(Page::Guide),
        "changelog" => Msg::RoutePage(Page::Changelog),
    };

    seed::App::build(Model::default(), update, view)
        .routes(routes)
        .finish()
        .run();
}
```
This syntax resembles that of the `attrs!` and `style!` macros, but uses commas
for separation.

To make landing-page routing work, configure your server so that all three of these paths point towards the app,
or that any (sub)path points towards it, instead of returning an error. The `serve.py` script
included in the quickstart repo and examples is set up for this. Once this is configured, intial 
routing on page load will work as expected: The page will load with the default state, then immediately 
trigger the update prescribed by the RoutePage message.

In order to trigger our route change through in-app naviation (eg clicking a link or pushing a button), include
logic like this in the update function:
```rust
#[derive(Clone)]
enum Msg {
    ChangePage(seed::App<Msg, Model>, u32),
    RoutePage(u32),
}

fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::ChangePage(state, page) => {
            // An enum, with a to_string() method might be a more elegant way
            // to store page state.
            let page_name = match page {
                0 => "",
                1 => "guide",
                2 => "changelog"
            };
            Render(seed::push_route(state, page_name, Msg::RoutePage(page)))
        },
        // This is separate, because in-app naviation needs to call push_route,
        // but we don't want to call it from browser navigation. (eg back button)
        Msg::RoutePage(page) => Model {page, ..model},
}
```
[seed::push_route](https://docs.rs/seed/0.1.8/seed/fn.push_route.html) accepts three single parameters:
a `seed::App`, a path &str corresponding to what will be appended to the url, and the message that handles
the state change. It sets up the routing, updates the model with the message you pass, and returns this
 updated model. In practice, these page_name, message combos will match your landing page routing config,
but they doesn't have to. You can push whatever you'd like dynamically. These will work for page navigation
and url display, but won't work for landing pages unless included in `.routes(routes)` described above.

When a page is loaded or browser naviation occurs (eg back button), Seed searches each of the route map keys for 
a matching path name (url suffix). If it finds one,
it updates the model based on its associated message. If not, no action is taken. 
In our example, we assume the model initialized to page=0, for the homepage.

Notice how we keep ChangePage and RoutePage separate in our example: RoutePage performs
the action associated with routing, while ChangePage updates our route history, then
recursively calls RoutePage. If you were to attempt this in the same message, each
browser navigation event would add a redundant route history entry, interfering with navigation. `seed::push_route`
calls RoutePage from ChangePage. We call ChangePage from an in-app navigation event, like this:

```rust
h2![ simple_ev(Ev::Click, Msg::ChangePage(state, 1)), "Guide" ]
```

Dynamic landing-page routes are not yet supported, but you may be able to populate the paths you
need ahead of time in the route map:
```rust
let mut routes = routes!{
    "guide" => Msg::RoutePage(Page::Guide),
    "changelog" => Msg::RoutePage(Page::Changelog),
};

for guide_page in 0..12 {
    routes.insert(
        "guide/".to_string() + &guide_page.to_string(),
        Msg::RouteGuidePage(guide_page)
    );
}
```
