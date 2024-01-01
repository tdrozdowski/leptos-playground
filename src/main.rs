use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! {<App/>})
}

#[component]
fn App() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));
    let on_input = move |ev| set_value.set(event_target_value(&ev).parse::<i32>());
    view! {
        <h1>"Error Handling"</h1>
        <label>"Type a number (or something that is not a number!)"
            <input type="number" on:input=on_input/>
            <ErrorBoundary
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors:"</p>
                        <ul>
                            { move || errors.get()
                                        .into_iter()
                                        .map(|(_, e)| view! {<li>{e.to_string()}</li>})
                                        .collect::<Vec<_>>()
                            }
                        </ul>
                    </div>
                }
            >
                <p>"You entered: "<strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}
