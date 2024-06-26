use leptos::{component, create_memo, create_signal, logging, view, CollectView, For, IntoView, Signal, SignalGet as _, SignalUpdate, SignalWith, WriteSignal};

#[component]
/// A simple counter component.
fn Counter(
    /// The current count.
    #[prop(into)]
    count: Signal<u16>,
    /// A signal to update the count.
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
pub fn StaticIteratorComponent(
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
pub fn DynamicIteratorComponent(
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

#[derive(Debug, Clone)]
struct MockDatabaseEntry {
    key: String,
    value: i32,
}

#[component]
pub fn AdvancedIterator() -> impl IntoView {

    let (data, set_data) = create_signal(vec![
        MockDatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        MockDatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        MockDatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        }
    ]);

    view! {
        <button on:click=move |_| {
            set_data.update(|data| {
                for row in data {
                    row.value *= 2;                                
                }
            });
            logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        <For 
            each=move || data.get().into_iter().enumerate()
            key=|(_, state)| state.key.clone()
            children=move |(index, _)| {
                let value = create_memo(move |_| {
                    data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
                });
                view!{
                    <p>{value}</p>
                }
            }
        />
    }
}
