use leptos::*;
use crate::components::analytics::*;

#[component]
pub fn Trinkets() -> impl IntoView {
    view! {
        <GoogleAnalytics />

        <div class="max-w-5xl mx-auto px-4 py-8">
            // Product name at the top
            <span class="flex items-center text-gray-100 mb-8">
                <span class="h-px flex-1 bg-white text-gray-100"></span>
                <span class="shrink-0 px-6 text-gray-100">memento-0001</span>
                <span class="h-px flex-1 bg-white text-gray-100"></span>
            </span>

            // Main container: 2 columns, each taking half the width
            <div class="flex items-start gap-4 visible-canvas">
                // Left: Spline 3D model viewer
                <div class="w-1/2 flex items-center justify-center">
                    <script type="module" src="https://unpkg.com/@splinetool/viewer@1.9.59/build/spline-viewer.js"></script>
                    <spline-viewer
                        url="https://prod.spline.design/9529ZhFYki4qj9Ue/scene.splinecode"
                        class="h-auto w-full" // Adjust as needed
                    ></spline-viewer>
                </div>

                // Right: Product description, bullet points, waitlist
                <div class="w-1/2 text-gray-100">
                    <h2 class="text-xl font-semibold mb-2">memento-0001 clock</h2>
                    <p class="mb-4">
                        E-Ink tabletop clocks that display how much you have left to live, the current time,
                        and how many weeks are left in a year. That is about it. Keeping it on my desk all the time
                        helps me focus on the current moment and remember how mortal we are and how little time is left.
                    </p>

                    <p class="mb-4 font-semibold">Specs:</p>
                    <ul class="list-disc ml-5 space-y-1 mb-4">
                        <li>E-Ink Display</li>
                        <li>Touch display to set the current date and birthdate</li>
                        <li>esp32</li>
                        <li>Type-C for power</li>
                        <li>
                            Even without power, the e-ink screen retains its display,
                            so you can carry it around and always remember how mortal you are.
                        </li>
                    </ul>

                    <p class="mb-4">
                        You can subscribe to the waitlist below. This was done as my hobby project,
                        but if enough people want it, I will consider producing and shipping them.
                    </p>


                    <p class="font-semibold mb-2">Subscribe to the waitlist:</p>
                    <div class="flex items-center">
                        <script async data-uid="fd36c683bd" src="https://alexkaravaev.kit.com/fd36c683bd/index.js"></script>
                    </div>
                </div>
            </div>

            // Optional: Substack feed or other content below
            <div id="substack-feed-embed" class="space-y-5.5 mt-8"></div>
        </div>
    }
}
