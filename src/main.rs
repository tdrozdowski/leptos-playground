use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! {<App/>})
}

#[component]
fn App() -> impl IntoView {
    view! {
        <TakesChildren render_prop=|| view! {<p>"Hi There!"</p>}>
            "Some Text"
            <span>"A span"</span>
        </TakesChildren>
    }
}

#[component]
fn TakesChildren<F, IV>(render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h2>"Render Prop"</h2>
        {render_prop()}

        <h2>"Children"</h2>
        {children()}

        <h3>"Wraps Children"</h3>
        <WrapsChildren>
            "A"
            "B"
            "C"
        </WrapsChildren>
    }
}

#[component]
fn WrapsChildren(children: Children) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! {<li>{child}</li>})
        .collect_view();

    view! {
        <ul>{children}</ul>
    }
}
