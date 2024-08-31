
use leptos::*;
#[cfg(target_arch = "wasm32")]
use crate::scene;

#[component]
fn Project(link: String, title: String, description: String) -> impl IntoView {
    view! {
        <a href=format!("{link}") class="object-contain group relative block h-64 sm:h-66 lg:h-96 min-h-full" target="_blank">
          <span class="absolute inset-0 border-2 border-dashed border-black"></span>

          <div
            class="relative flex h-full transform items-end border-2 border-black bg-white transition-transform group-hover:-translate-x-2 group-hover:-translate-y-2"
          >
            <div
              class="p-4 !pt-0 transition-opacity group-hover:absolute group-hover:opacity-0 sm:p-6 lg:p-8"
            >
              <h2 class="mt-4 text-xl font-medium sm:text-2xl">{&title}</h2>
            </div>

            <div
              class="absolute p-4 opacity-0 transition-opacity group-hover:relative group-hover:opacity-100 sm:p-6 lg:p-8"
            >
              <h3 class="mt-4 text-xl font-medium sm:text-2xl">{title}</h3>

              <p class="mt-4 text-sm sm:text-xs text-gray-900 break-words">
                  {description}
                </p>

              <p class="mt-8 font-bold sm:text-base text-gray-900">Read more</p>
            </div>
          </div>
        </a>
    }
}

#[component]
pub fn Main() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(async move {
        scene::scene().await;
    }); 
    view! {
        <div class="font-eurostile relative isolate px-6 pt-14 lg:px-8 flex flex-col min-h-full">
            <div class="min-w-full flex items-start gap-4 min-h-full visible-canvas">
                <canvas id="myCanvas" class="visible-canvas"></canvas>
                
                <div>
                <h3 class="text-lg/tight font-medium text-gray-100">Alex Karavaev</h3>
                <p class="mt-0.5 text-gray-100">
                
                </p>
                <p class="mt-0.5 text-gray-100">
                    Robotics engineer, human, I love when my work not only moves pixels around, but also atoms. 
                    You can book 15 mins with me <a class="under" href="https://calendly.com/alexkaravev/coffee-chat" target="_blank">here</a> to chat, if there is agenda set.
                </p>
                </div>

            </div>
            <span class="flex items-center text-gray-100">
            <span class="h-px flex-1 bg-white text-gray-100"></span>
            <span class="shrink-0 px-6 text-gray-100">Life timeline</span>
            <span class="h-px flex-1 bg-white text-gray-100"></span>
            </span>        

            <div class="min-w-full flex min-h-full items-center justify-center">
                <section class="relative flex flex-col justify-center bg-black overflow-hidden">
                <div class="w-full max-w-6xl mx-auto px-4 md:px-6">
                    <div class="flex flex-col justify-center divide-y divide-slate-200 [&>*]:py-16">
            
                        <div class="w-full max-w-3xl mx-auto">
                        
                            <div class="-my-6">
                                <p> <a class="under" href="docs/resume.pdf" target="_blank">My full CV.</a></p>
                                <div class="relative pl-8 sm:pl-32 py-6 group">
                                    <div class="flex flex-col sm:flex-row items-start mb-1 group-last:before:hidden before:absolute before:left-2 sm:before:left-0 before:h-full before:px-px before:bg-slate-300 sm:before:ml-[6.5rem] before:self-start before:-translate-x-1/2 before:translate-y-3 after:absolute after:left-2 sm:after:left-0 after:w-2 after:h-2 after:bg-indigo-600 after:border-4 after:box-content after:border-slate-50 after:rounded-full sm:after:ml-[6.5rem] after:-translate-x-1/2 after:translate-y-1.5">
                                        <img class="sm:absolute left-0 translate-y-0.5 inline-flex items-center justify-center font-semibold uppercase" src="/images/itmo.png" alt="a lighthouse"/>
                                        <div class="text-xl font-bold text-gray-100">
                                        Completing tutorials on life
                                        </div>
                                    </div>
                                    <div class="text-gray-100">
                                        During these years, I was studying and interning. I completed a Bachelors degree in Robotics and began a Masters in Machine Learning, but eventually chose to drop out. 
                                    </div>
                                </div>
                                
                                <div class="relative pl-8 sm:pl-32 py-6 group">
                                    <div class="flex flex-col sm:flex-row items-start mb-1 group-last:before:hidden before:absolute before:left-2 sm:before:left-0 before:h-full before:px-px before:bg-slate-300 sm:before:ml-[6.5rem] before:self-start before:-translate-x-1/2 before:translate-y-3 after:absolute after:left-2 sm:after:left-0 after:w-2 after:h-2 after:bg-indigo-600 after:border-4 after:box-content after:border-slate-50 after:rounded-full sm:after:ml-[6.5rem] after:-translate-x-1/2 after:translate-y-1.5">
                                        <img class="sm:absolute left-0 translate-y-0.5 inline-flex items-center justify-center font-semibold uppercase" src="/images/jb.png" alt="a lighthouse"/>
                                        <div class="text-xl font-bold text-gray-100">JetBrains</div>
                                    </div>
                                    <div class="text-gray-100">I was the founding engineer at the Museum of the Future, where I developed exhibitions featuring robotic manipulators and small self-driving cars.</div>
                                </div>
                                
                                <div class="relative pl-8 sm:pl-32 py-6 group">
                                    <div class="flex flex-col sm:flex-row items-start mb-1 group-last:before:hidden before:absolute before:left-2 sm:before:left-0 before:h-full before:px-px before:bg-slate-300 sm:before:ml-[6.5rem] before:self-start before:-translate-x-1/2 before:translate-y-3 after:absolute after:left-2 sm:after:left-0 after:w-2 after:h-2 after:bg-indigo-600 after:border-4 after:box-content after:border-slate-50 after:rounded-full sm:after:ml-[6.5rem] after:-translate-x-1/2 after:translate-y-1.5">
                                        <img class="sm:absolute left-0 translate-y-0.5 inline-flex items-center justify-center font-semibold uppercase" src="/images/mag.png" alt="a lighthouse"/>
                                        <div class="text-xl font-bold text-gray-100">Magazino</div>
                                    </div>
                                    <div class="text-gray-100">Focused on robot simulation, CI/CD, and improving developer experience (DevEx).</div>
                                </div>      
                            </div>
                            
                        </div>
            
                    </div>
                </div>


                </section>
            </div>
            <span class="flex items-center text-gray-100">
            <span class="h-px flex-1 bg-white text-gray-100"></span>
            <span class="shrink-0 px-6 text-gray-100">Projects</span>
            <span class="h-px flex-1 bg-white text-gray-100"></span>
            </span>
            
            <div class="min-w-full flex min-h-full items-center justify-center">
                <section class="relative flex flex-col justify-center bg-black overflow-hidden">
                
                    <p> Most of my projects are open-source on <a class="under" href="https://github.com/alexkaravaev" target="_blank">"💾 Github"</a>, but here are a couple of projects that arent available there. </p>
                    <div class="grid p-24 place-items-center gap-4 grid-flow-row-dense lg:gap-8 mt-5.5 content-center">
                    <div class="h-22 rounded-lg bg-gray-200 min-w-full">
                        <Project
                            link="#".to_string()
                            title="Foodie AI".to_string()
                            description="Building a calorie tracker using only photos and AI nutritionists. This post will be more detailed in the future.".to_string()/>
                    </div>
                    <div class="h-22 rounded-lg bg-gray-200 min-w-full">
                        <Project
                            link="https://linkedin.com/company/ros-germany".to_string() 
                            title="Robotics meetups".to_string()
                            description="Started robotics meetups in Munich with ~3k online community".to_string()
                        />
                    </div>
                    </div>
               
                </section>  
            </div>

            // https://de.linkedin.com/company/ros-germany
            <span class="flex items-center text-gray-100">
            <span class="h-px flex-1 bg-white text-gray-100"></span>
            <span class="shrink-0 px-6 text-gray-100">Acknowledgements</span>
            <span class="h-px flex-1 bg-white text-gray-100"></span>
            </span>

            <div class="min-w-full grid w-full max-w-6xl mx-auto px-4 md:px-6 py-4 space-y-12 min-h-full items-center justify-center">
                <p> Thanks for great <a class="under" href="https://departuremono.com" target="_blank"> font </a> and leptos website <a class="under" href="https://github.com/khuedoan/blog/tree/master" target="_blank">template</a> </p>
                <p> This <a class="under" href="https://github.com/AlexKaravaev/blog" target="_blank">site</a> is made with <a class="under" href="https://leptos.dev/" target="_blank">leptos</a>. Lovely that you can make frontend apps now with Rust, god bless WASM.</p>
            </div>
      
        </div>
    }
}
