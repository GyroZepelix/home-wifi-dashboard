use leptos::{
    component, create_signal, mount_to_body, view, CollectView, For, IntoView, Signal, SignalGet, SignalUpdate, WriteSignal
};

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
        <StaticIteratorComponent/>
        <br/>
        <DynamicIteratorComponent initial_lenght=3/>
    }
}

/// A progress bar component.
#[component]
fn ProgressBar(
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

#[component]
fn Counter(
    #[prop(into)]
    count: Signal<u16>,
    set_count: WriteSignal<u16>,
) -> impl IntoView {
    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            {count}
        </button>
    }
}

/// A list of counters, without the ability
/// to add or remove any.
#[component]
fn StaticIteratorComponent(
    /// How many counters to display.
    #[prop(default = 5)] lenght: u16
) -> impl IntoView {
    let counters = (1..=lenght).map(|idx| create_signal(idx));

    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <Counter count=count set_count=set_count/>
                </li>
            }
        })
        .collect_view();

    view! {
        <ul>{counter_buttons}</ul>
    }
}

/// A list of counters that allows you to add or remove counters.
#[component]
fn DynamicIteratorComponent(
    /// The number of counters to begin with
    initial_lenght: u16,
) -> impl IntoView {
    let mut next_counter_id = initial_lenght;

    let initial_counters = (0..initial_lenght)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        let sig = create_signal(next_counter_id + 1);
        set_counters.update(move |counters| {
            counters.push((next_counter_id, sig))
        });

        next_counter_id += 1;
    };

    let remove_counter = move |id| {
        set_counters.update(|cntr| {
            cntr.retain(|(counter_id, _)| *counter_id != id)
        })
    };

    view! {
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                <For
                    each=move || counters.get()
                    key=|counter| counter.0
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <Counter count=count set_count=set_count/>
                                <button on:click=move |_| {
                                    remove_counter(id)
                                }>
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
