use crate::error_template::{AppError, ErrorTemplate};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    use crate::components::layout::header::Header;
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <html lang="en">
            <head>
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/martingraham-blog.css"/>
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
        <link rel="preload" href="https://fonts.googleapis.com/css2?family=Open+Sans:ital,wght@0,300..800;1,300..800&family=SUSE:wght@100..800&display=swap" rel="stylesheet"/>
        // favicon
        <link
           rel="icon"
           type="image/svg+xml"
           href="data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%27%20width%3D%2764%27%20height%3D%2764%27%3E%3Crect%20width%3D%2764%27%20height%3D%2764%27%20fill%3D%27%23444%27%2F%3E%3Ctext%20x%3D%2750%25%27%20y%3D%2750%25%27%20font-size%3D%2730%27%20fill%3D%27%23fff%27%20text-anchor%3D%27middle%27%20dominant-baseline%3D%27middle%27%3EMG%3C%2Ftext%3E%3C%2Fsvg%3E"
        />

        // sets the document title
        <Title text="martingraham.dev"/>
        </head>
         <body>
            <Header />
            <main>
                <article>
                    <Router >
                        <Routes fallback=|| {
                            let mut outside_errors = Errors::default();
                            outside_errors.insert_with_default_key(AppError::NotFound);
                            view! { <ErrorTemplate outside_errors/> }.into_view()
                            }>
                                <Route path=path!("") view=PostList/>
                        </Routes>
                    </Router>
                </article>
            </main>
            <footer>
                <p>Built with <a href="https://www.leptos.dev" target="_blank">Leptos</a></p>
          </footer>
      </body>
    </html>
    }
}

#[component]
fn PostList() -> impl IntoView {
    use crate::components::post::post::Post;
    view! {
        <Post markdown_path={"src/posts/first.md".to_string()}/>
    }
}
