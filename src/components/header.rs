use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    let nav_items = [("about", "/about"), ("thoughts", "/thoughts"), ("resources", "/resources")];

    view! {
        <header class="relative z-50 bg-black sticky top-0" >

            <nav
                class="mx-auto flex max-w-7xl items-center justify-between p-6 lg:px-8"
                aria-label="Global"
            >
                
                <div class="flex gap-x-6 lg:gap-x-12">
                    {nav_items
                        .iter()
                        .map(|(name, href)| {
                            view! {
                                <a
                                    class="under text-sm font-semibold leading-6 text-gray-100"
                                    href=href.to_string()
                                >
                                    {name.to_string()}
                                </a>
                            }
                        })
                        .collect_view()}
                </div>
            </nav>
        </header>
    }
}
