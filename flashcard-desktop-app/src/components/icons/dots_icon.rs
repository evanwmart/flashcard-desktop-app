use leptos::*;

#[component]
pub fn DotsIcon(class: Option<&'static str>) -> impl IntoView {
    let class_string = class.unwrap_or("");

    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="currentColor" viewBox="0 0 256 256"
class={class_string}><path d="M140,128a12,12,0,1,1-12-12A12,12,0,0,1,140,128Zm56-12a12,12,0,1,0,12,12A12,12,0,0,0,196,116ZM60,116a12,12,0,1,0,12,12A12,12,0,0,0,60,116Z"></path></svg>    }
}