use std::{
    fs::{self},
    io::Read,
};

use leptos::*;
use markdown;

#[component]
pub fn Post(markdown_path: String) -> impl IntoView {
    let markdown_str =
        fs::read_to_string(markdown_path).unwrap_or("didn't read from file".to_string());
    let html = markdown::to_html(&markdown_str);
    view! {

       <div>
           <div inner_html={html} />
       </div>
    }
}
