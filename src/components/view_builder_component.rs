use leptos::{component, html::{em, p, strong}, IntoView};

#[component]
pub fn MacrolessComponent() -> impl IntoView {
    // This is a macro-less alternative to the follwing
    // view! {
    //     <p>
    //         <em>"Big, "</em>
    //         <strong>"bold "</strong>
    //         "text"
    //     </p>
    // }
    p().child( (em().child("Big, "), strong().child("bold "), "text") )
}
