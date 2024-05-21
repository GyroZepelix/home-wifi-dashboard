use leptos::{component, create_signal, event_target_value, view, CollectView, ErrorBoundary, IntoView, SignalGet, SignalSet};

#[component]
///Component showing the rendering of Result<T>
pub fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev| set_value.set(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="text" on:input=on_input/>
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect_view()
                            }
                        </ul>
                    </div>
                }
            >
                <p>"You entered " <strong>{value}</strong></p>
            </ ErrorBoundary>
        </label>
    }


}
