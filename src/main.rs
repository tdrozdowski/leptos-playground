use gloo_timers::future::TimeoutFuture;
use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! {<App/>})
}

async fn important_api_call(id: usize) -> String {
    TimeoutFuture::new(1_000).await;
    match id {
        0 => "Alice",
        1 => "Bob",
        2 => "Carol",
        _ => "User Not Found",
    }.to_string()
}

#[component]
fn App() -> impl IntoView {
    let (tab, set_tab) = create_signal(0);
    let user_data = create_resource(move || tab.get(),  |tab| async move  { important_api_call(tab).await});
    view! {
        <div class="buttons">
            <button
                on:click=move |_| set_tab.set(0)
                class:selected=move || tab.get() == 0>
                "Tab A"
            </button>
            <button
                on:click=move |_| set_tab.set(1)
                class:selected=move || tab.get() == 1>
                "Tab B"
            </button>
            <button
                on:click=move |_| set_tab.set(2)
                class:selected=move || tab.get() == 2>
                "Tab C"
            </button>
            {move || if user_data.loading().get() {
                        "Loading..."
                    } else {
                        ""
                    }
            }
        </div>
        <Transition
            fallback=move || view! {<p>"Loading..."</p>}
        >
            <p>
                {move || user_data.get()}
            </p>
        </Transition>
    }

}
