use leptos::{component, create_signal, mount_to_body, view, IntoView, ReadSignal, Signal, SignalGet, SignalUpdate};

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
    let double_count = move || count.get() * 2;

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 10);
            }
        >
            "Click me"
        </button>
        <ProgressBar progress=count/>
        <ProgressBar progress=Signal::derive(double_count)/>
    }
}

/// A progress bar component.
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)] max: u16,
    /// The current progress value.
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}
