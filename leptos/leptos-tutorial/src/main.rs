use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click= move |_| {
                set_count.update(|n| *n += 1);
            }
            class:red=move || count() % 2 == 1
        >
            "Click me: "
            {count}
        </button>
    }
}
