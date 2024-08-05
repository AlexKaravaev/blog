use chrono::{Datelike, Utc};
use leptos::*;
use leptos_icons::*;
use icondata;

struct Social<'a> {
    name: &'a str,
    url: &'a str,
    icon: icondata::Icon,
}


#[component]
fn SocialIcons() -> impl IntoView {
    let socials = [
        Social {
            name: "Mail",
            url: "mailto:alexkaravaev@alexkaravaev.xyz",
            icon: icondata::AiMailFilled,
        },
        Social {
            name: "GitHub",
            url: "https://github.com/AlexKaravaev",
            icon: icondata::AiGithubFilled,
        },
    ];

    view! {
        <div class="mt-10 flex justify-center space-x-5">
            {socials
                .iter()
                .map(|social| {
                    view! {
                        <a
                            key=social.name
                            href=social.url
                            target="_blank"
                            class="text-gray-100 hover:text-gray-500"
                        >
                            <span class="sr-only">{social.name}</span>
                            <Icon icon=social.icon class="h-6 w-6"/>
                        </a>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-black">
            <div class="mx-auto max-w-7xl overflow-hidden py-20 px-6 sm:py-24 lg:px-8 bg-black">
                <SocialIcons/>
                <p class="mt-5 text-center text-sm leading-5 text-gray-100">
                    {format!("© {} a.k", Utc::now().year())}
                </p>
                <p class="mt-5 text-center text-sm leading-5 text-gray-100">
                    
                    <span class="custom-glyph-planet"></span>"Made on Earth with love"
                </p>
            </div>
        </footer>
    }
}
