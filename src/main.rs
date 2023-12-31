use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! {<App/>})
}

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count.get() * 2;
    let length = 5;
    let values = vec![1, 2, 3, 4, 5];
    let counters = (0..length).map(|idx| create_signal(idx));
    let counter_buttons = counters.map(|(count, set_count)| {
        view! {
            <button on:click=move |_| { set_count.update(|n| *n += 1);}
            >
                {move || count.get()}
            </button>
        }
    }).collect_view();

    view! {
        <div>
            <button on:click=move |_| { set_count.update(|n| *n += 1);}
                    class:red=move || count.get() % 2 == 1
        >
                "Click me:"
                {move || count.get()}
            </button>
        </div>
        <ProgressBar progress=count/>
        <ProgressBar progress=double_count/>
        <p>
            {values.clone()}
        </p>
        <ul>
            {values.into_iter()
                .map(|v| view!{<li>{v}</li>})
                .collect_view()
            }
        </ul>
        <ul>{counter_buttons}</ul>
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


