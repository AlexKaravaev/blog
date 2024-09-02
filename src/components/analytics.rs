use leptos::*;
use leptos_meta::Script;    

#[component]
pub fn GoogleAnalytics() -> impl IntoView {
    view! {
        <script async src="https://www.googletagmanager.com/gtag/js?id=G-Y246ZKRXWH"></script>
        <Script>
        "
          window.dataLayer = window.dataLayer || [];
          function gtag(){dataLayer.push(arguments);}
          gtag('js', new Date());
        
          gtag('config', 'G-Y246ZKRXWH');
        "
        </Script>
    }
}

