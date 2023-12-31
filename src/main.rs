use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! {<App/>})
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,
}

#[component]
fn App() -> impl IntoView {
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "one".to_string(),
            value: create_rw_signal(10),
        },
        DatabaseEntry {
            key: "two".to_string(),
            value: create_rw_signal(20),
        },
        DatabaseEntry {
            key: "three".to_string(),
            value: create_rw_signal(30)
        },
    ]);
    view! {
        <button on:click=move |_| {
            set_data.update(|data| {
                for row in data {
                    row.value.update(|n| *n *=2);
                }
            });
            logging::log!("data: {:?}", data.get());
        }>
        "Update Values"
        </button>
        <For
            each = move || data.get()
            key = |state| state.clone()
            let:child
        >
            <p>{move || child.value}</p>
        </For>
    }
}

#[component]
fn ProgressBar(
    #[prop(default = 100)]
    max: i32,
    #[prop(into)]
    progress : Signal<i32>
) -> impl IntoView
{
    view! {
        <progress max=max value=progress/>
    }
}


