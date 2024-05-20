use leptos::{component, create_signal, view, IntoView, Signal, SignalGet, SignalUpdate};

use crate::components::{iterator_components::{AdvancedIterator, DynamicIteratorComponent, StaticIteratorComponent}, progress_bar::ProgressBar};

#[component]
pub fn App() -> impl IntoView {
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
        <StaticIteratorComponent/>
        <br/>
        <DynamicIteratorComponent initial_lenght=3 />
        <AdvancedIterator />
    }
}

