use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    use crate::components::layout::header::Header;
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/martingraham-blog.css"/>
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
        <link href="https://fonts.googleapis.com/css2?family=SUSE:wght@100..800&display=swap" rel="stylesheet"/>

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
                    <div class="content">
                        <Routes>
                            <Route path="" view=PostList/>
                        </Routes>
                    </div>
                </div>
            </main>
        </Router>
    }
}

#[component]
fn PostList() -> impl IntoView {
    use crate::components::post::post::Post;
    view! {
        <Post markdown_path={"src/posts/first.md".to_string()}/>
    }
}
