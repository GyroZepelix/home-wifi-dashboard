use leptos::{component, create_signal, mount_to_body, view, IntoView, ReadSignal, SignalUpdate};

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
                set_count.update(|n| *n += 10);
            }
        >
            "Click me"
        </button>
        <ProgressBar progress=count/>
    }
}

#[component]
fn ProgressBar(
    progress: ReadSignal<i32>,
    #[prop(default = 100)]
    max: u16,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}
