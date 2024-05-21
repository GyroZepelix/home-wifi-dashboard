use leptos::{component, create_signal, ev::MouseEvent, provide_context, use_context, view, Callable, Callback, IntoView, SignalUpdate, WriteSignal};

#[component]
pub fn ButtonAContainer() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <p>"Toggled? " {toggled}</p>
        <ButtonA on_click=move |_| set_toggled.update(|value| *value = !*value)/>
    }
}

#[component]
pub fn ButtonA(#[prop(into)] on_click: Callback<MouseEvent>) -> impl IntoView
{
    view! {
        <button on:click=move |ev| on_click.call(ev)>
            "Toggle"
        </button>
    }
}


#[component]
pub fn ContextContainer() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);

    provide_context(set_toggled);

    view! {
        <p>"Toggled? " {toggled}</p>
            <Layout/>
    }
}

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <header>
            <h1>"My Page"</h1>
        </header>
        <main>
            <Content/>
        </main>
    }
}

#[component]
pub fn Content() -> impl IntoView {
    view! {
        <div class="content">
            <ButtonD/>
        </div>
    }
}

#[component]
pub fn ButtonD() -> impl IntoView {
    let setter = use_context::<WriteSignal<bool>>()
        .expect("setter not provided");

    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle"
        </button>
    }
}
