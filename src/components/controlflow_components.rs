use leptos::{component, create_signal, view, IntoView, Show, SignalGet};


#[component]
pub fn SimpleControlFlow() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value.get() & 1 == 1;

    view! {
        <p>
        {move || if is_odd() {
             "Odd"
        } else {
            "Even"
        }}
        </p>
    }
}

#[component]
pub fn AdvancedControlFlow() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    view! {
        <Show
            when=move || { value.get() > 5 }
            fallback=|| view! { "Small" }
        >
            "Big" 
        </Show>
    }
}
