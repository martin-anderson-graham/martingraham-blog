use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div>
            <h1>blog.martingraham.dev</h1>
                <div class="subnav-links">
                    <a>Posts</a>
                    <a href="https://github.com/martin-anderson-graham/martingraham-blog" target="_blank">Github</a>
                    <a href="https://www.martingraham.dev" target="_blank">About me</a>
                </div>
            </div>
    }
}
