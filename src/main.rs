use leptos::*;
use leptos_router::*;

fn main() {
    leptos::mount_to_body(|| view! {<App/>})
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <h1>"Contact App"</h1>
            <nav>
                <h2>"Navigation"</h2>
                <a href="/">"Home"</a>
                <a href="/contacts">"Contacts"</a>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=|| view!{
                        <h3>"Home"</h3>
                    }/>
                    <Route path="/contacts" view=ContactList>
                        <Route path=":id" view=ContactInfo>
                            <Route path="" view=|| view! {
                                <div class="tab">
                                    "(Contact info)"
                                </div>
                             }/>
                            <Route path="conversations" view=|| view! {
                                <div class="tab">
                                    "(Conversations)"
                                </div>
                            }/>
                        </Route>
                        <Route path="" view=|| view! {
                            <div class="select-user">
                                "Select a user to view contact info."
                            </div>
                        }/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}


#[component]
fn ContactList() -> impl IntoView {
    view! {
        <div class="contact-list">
            <div class="contact-list-contacts">
                <h3>"Contacts"</h3>
                <A href="alice">"Alice"</A>
                <A href="bob">"Bob"</A>
                <A href="steve">"Steve"</A>
            </div>
            <Outlet/>
        </div>
    }
}
#[component]
fn ContactInfo() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User Not Found",
    };

    view! {
        <div class="contact-info">
            <h4>{name}</h4>
            <div class="tabs">
                <A href="" exact=true>"Contact Info"</A>
                <A href="conversations">"Conversations"</A>
            </div>
            <Outlet/>
        </div>
    }
}
