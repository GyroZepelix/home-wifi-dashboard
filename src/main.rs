use leptos::{component, create_signal, mount_to_body, view, IntoView, SignalGet, SignalUpdate};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    });
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            class:red=move || count.get() % 2 == 1
        >
            "Click me: "
            {count}
        </button>
    }
}
