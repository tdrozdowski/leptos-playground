use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! {<App/>})
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
fn App() -> impl IntoView {
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "one".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "two".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "three".to_string(),
            value: 30
        },
    ]);
    view! {
        <button on:click=move |_| {
            set_data.update(|data| {
                for row in data {
                    row.value *=2;
                }
            });
            logging::log!("data: {:?}", data.get());
        }>
        "Update Values"
        </button>
        <For
            each = move || data.get().into_iter().enumerate()
            key = |(_, state)| state.key.clone()
            children = move |(index, _)| {
               let value = create_memo(move |_| {
                  data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
                });
                view! {
                    <p>{value}</p>
                }
            }/>
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


