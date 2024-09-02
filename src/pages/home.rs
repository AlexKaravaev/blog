use leptos::*;

use crate::components::main::*;
use crate::components::analytics::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <GoogleAnalytics/>
        <Main/>
    }
}
