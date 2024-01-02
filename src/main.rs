use leptos::ev::MouseEvent;
use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! {<App/>})
}

#[component]
fn App() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);

    provide_context(set_toggled);

    view! {
        <p>"Toggled? " {toggled}</p>
        <Layout />
    }
}

#[component]
fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
        "Toggle"
        </button>
    }
}

#[component]
fn ButtonB<F>(on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
        <button on:click=on_click>
            "Toggle"
        </button>
    }
}

#[component]
fn Layout() -> impl IntoView {
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
fn Content() -> impl IntoView {
    view! {
        <div class="content">
            <ButtonD/>
        </div>
    }
}

#[component]
fn ButtonD() -> impl IntoView {
    let setter = use_context::<WriteSignal<bool>>().expect("to have found the setter provided.");
    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle"
        </button>
    }
}
