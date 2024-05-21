use leptos::{component, create_signal, view, IntoView, Signal, SignalGet, SignalUpdate};

use crate::components::{controlflow_components::{AdvancedControlFlow, SimpleControlFlow}, error_handling_components::NumericInput, input_components::{FormInput, TextInput}, iterator_components::{AdvancedIterator, DynamicIteratorComponent, StaticIteratorComponent}, parent_child_coms_components::{ButtonAContainer, ContextContainer}, passing_children_components::{TakesChildren, WrapsChildren}, progress_bar::ProgressBar};

#[component]
/// The root of the app
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
        <hr/>
        <StaticIteratorComponent/>
        <DynamicIteratorComponent initial_lenght=3 />
        <AdvancedIterator />
        <hr/>
        <TextInput />
        <FormInput />
        <hr />
        <SimpleControlFlow />
        <AdvancedControlFlow />
        <hr />
        <NumericInput />
        <hr />
        <ButtonAContainer />
        <ContextContainer />
        <hr />
        <TakesChildren render_prop=|| view! { <p>"Hi, there!"</p> }>
            // these get passed to `children`
            "Some text"
            <span>"A span"</span>
        </TakesChildren>
        <WrapsChildren>
            "A"
            "B"
            "C"
        </WrapsChildren>
        <hr/>
    }
}

