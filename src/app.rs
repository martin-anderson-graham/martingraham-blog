use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    use crate::components::layout::header::Header;
    use crate::components::layout::sidebar::Sidebar;
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/martingraham-blog.css"/>

        // sets the document title
        <Title text="Welcome to prediction-ledger."/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <div class="layout-container">
                    <div class="header">
                        <Header />
                    </div>
                    <div class="sidebar">
                        <Sidebar />
                    </div>
                    <div class="content">
                        <Routes>
                            <Route path="" view=HomePage/>
                        </Routes>
                    </div>
                </div>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to prediction-ledger!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
