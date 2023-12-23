use leptos::{*, html::Input};
use web_sys::SubmitEvent;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled".to_string());
    let input_element: NodeRef<Input> = create_node_ref();
    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();

        let value = input_element()
            .expect("<input> to exist")
            .value();
        set_name(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text" 
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit" />
        </form>
        <p>"Name is: "{name}</p>
    }
}
