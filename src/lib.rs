pub mod app;
pub mod scene;
pub mod components;
pub mod error_template;
pub mod pages;

#[cfg(feature = "ssr")]
pub mod file_serve;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    // console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));


    leptos::mount_to_body(App);

}
