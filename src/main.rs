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
    let (x, set_x) = create_signal(0);
    view! {
        <button
            on:click=move |_| {
                set_x.update(|n| *n += 10);
            }
            style:left=move || format!("{}px", x.get() + 100)
            style="position: absolute"
            style:background-color=move || format!("rgb({}, {}, 100)", x.get(), 100)
            style:max-width="400px"
            style=("--columns", x)
        >
            "Click to Move"
        </button>
        <progress
            max="50"
            value=x
        />
    }
}
