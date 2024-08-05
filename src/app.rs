use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::error_template::{AppError, ErrorTemplate};

use crate::pages::home::*;
use crate::pages::resources::*;
use crate::pages::posts::*;

use crate::components::footer::*;
use crate::components::header::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/blog.css"/>
        <Title text="Alex Karavaev"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Header/>

            <main>
            <Link rel="preload" href="https://alexkaravaev.ck.page/5a87217522/index.js" as_="script"/>
            <Link rel="preload" href="https://substackapi.com/widget.js" as_="script"/>
            <Link rel="preload" href="https://substackapi.com/embeds/feed.js" as_="script"/>
    
                <Routes>
                    <Route path="/about" view=|| view! { <Home/> } ssr=SsrMode::Async/>
                    <Route path="/thoughts" view=|| view! { <Posts/> }/>
                    <Route path="/resources" view=|| view! { <Resources/> }/>
                    
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
