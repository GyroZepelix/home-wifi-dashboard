use leptos::{component, view, IntoView, Signal};

/// A progress bar component.
#[component]
pub fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// The current progress value.
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

