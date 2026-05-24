use icondata;
use leptos::*;
use leptos_icons::*;

struct HeaderSocial<'a> {
    name: &'a str,
    url: &'a str,
    icon: icondata::Icon,
}

#[component]
pub fn Header() -> impl IntoView {
    let nav_items = [
        ("about", "/about"),
        ("thoughts", "/thoughts"),
        ("trinkets", "/trinkets"),
        ("resources", "/resources"),
    ];
    let socials = [
        HeaderSocial {
            name: "LinkedIn",
            url: "https://www.linkedin.com/in/alexkaravaev/",
            icon: icondata::AiLinkedinFilled,
        },
        HeaderSocial {
            name: "X",
            url: "https://x.com/alex__karavaev",
            icon: icondata::BsTwitterX,
        },
    ];

    view! {
        <header class="relative z-50 bg-black sticky top-0" >

            <nav
                class="mx-auto flex max-w-7xl flex-wrap items-center justify-between gap-4 p-6 lg:px-8"
                aria-label="Global"
            >

                <div class="flex flex-wrap gap-x-5 gap-y-1 sm:gap-x-6 lg:gap-x-12">
                    {nav_items
                        .iter()
                        .map(|(name, href)| {
                            view! {
                                <a
                                    class="under text-sm font-semibold leading-6 text-gray-100"
                                    href=href.to_string()
                                    rel="external"
                                >
                                    {name.to_string()}
                                </a>
                            }
                        })
                        .collect_view()}
                </div>
                <div class="flex items-center gap-x-4">
                    {socials
                        .iter()
                        .map(|social| {
                            view! {
                                <a
                                    key=social.name
                                    href=social.url
                                    target="_blank"
                                    rel="external noopener noreferrer"
                                    class="text-gray-100 transition hover:text-gray-500"
                                    aria-label=social.name
                                >
                                    <span class="sr-only">{social.name}</span>
                                    <Icon icon=social.icon class="h-5 w-5"/>
                                </a>
                            }
                        })
                        .collect_view()}
                </div>
            </nav>
        </header>
    }
}
