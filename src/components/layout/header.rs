use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <h1>blog.martingraham.dev</h1>
                <nav>
                <ul>
                    <li><a>Posts</a></li>
                    <li><a href="https://github.com/martin-anderson-graham/martingraham-blog" target="_blank">Github</a></li>
                    <li><a href="https://www.martingraham.dev" target="_blank">About me</a></li>
                    </ul>
                </nav>
            </header>
    }
}
