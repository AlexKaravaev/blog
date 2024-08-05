use leptos::*;

#[component]
pub fn Posts() -> impl IntoView {
    view! {

        <div class="text-center">
            <h1 class="mt-2 py-20 text-3xl font-medium tracking-tight text-gray-100 sm:text-4xl">
                "thoughts"
            </h1>

            <p class="mt-0.5 text-gray-100">
                You can subscribe to my brain here:    
            </p>
            

            <div class="flex flex-col space-y-5.5">
                <div class="flex flex-row justify-center items-center">
                    <script async data-uid="5a87217522" src="https://alexkaravaev.ck.page/5a87217522/index.js"></script>
                </div>
                
                <div id="substack-feed-embed" class="space-y-5.5"></div>
            </div>

            <script>
            "
            window.CustomSubstackWidget = {
                substackUrl: 'alexkaravaev.substack.com',
                placeholder: 'example@gmail.com',
                buttonText: 'Subscribe',
                theme: 'custom',
                colors: {
                    primary: '#252525',
                    input: '#000000',
                    email: '#FFFFFF',
                    text: '#FFFFFF',
                },
                // Go to substackapi.com to unlock custom redirect

            };
            "
            </script>
            <script src="https://substackapi.com/widget.js" async></script>
            
            

            <script>
            "
                window.SubstackFeedWidget = {
                    substackUrl: 'alexkaravaev.substack.com'    ,
                    posts: 2,
                    colors: {
                    primary: '#FFFFFF',
                    secondary: '#FFFFFF',
                    background: '#000000',
                    }
                };
            "
            </script>
            <script src="https://substackapi.com/embeds/feed.js" async></script>
        </div>
    }
}

