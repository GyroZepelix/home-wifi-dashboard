use leptos::{component, create_node_ref, create_signal, event_target_value, html, view, IntoView, NodeRef, SignalSet};

#[component]
/// Simple text input with reactive variable
pub fn TextInput() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <input type="text"
            on:input=move |ev| {
                set_name.set(event_target_value(&ev))
            }
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
/// Input that asks for your name and uses a "form" to submit
pub fn FormInput() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontroller".to_string());

    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let value = input_element.get()
            .expect("<input> should be mounted")
            .value();

        set_name.set(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is:" {name}</p>
    }
}
