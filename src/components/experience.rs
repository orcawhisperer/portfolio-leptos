use icondata as i;
use leptos::*;
use leptos_icons::*;

use crate::app::{Experience, Position};

// TODO: Load from JSON

#[component]
pub fn WorkExperienceItem(key: usize, position: Position) -> impl IntoView {
    let (collapsed, setCollapsed) = create_signal(false);

    create_effect(move |_| logging::log!("collapsed = {}", collapsed.get()));

    view! {
        <div key={key} class="border-l-4 border-blue-500 p-4 mb-6 transition-all duration-300 transform hover:scale-105 cursor-pointer"
        on:click=move |_| setCollapsed.update(|collapsed: &mut bool| *collapsed = !*collapsed)>
        <h4 class="text-xl font-semibold mb-2 text-white shadow-text font-montserrat">
           {position.title.to_string()}
           <span
              class="inline-block ml-2 transform transition-transform duration-300"
              class=("rotate-90", move || collapsed.get() == true)
            //   class:rotate-90=move || !collapsed.get()
              >

              <Icon icon={i::BsChevronDown} class="text-white" />
           </span>
        </h4>
        <p class="text-gray-300 mb-2 shadow-text font-montserrat">
           {position.duration.to_string()}
        </p>

        <Show when=move || collapsed.get()>
            <ul class="list-disc pl-5 text-gray-300 shadow-text font-montserrat">
                {position.responsibilities.iter().enumerate().map(|(index, responsibility)| view! {
                    <li key={index}>{responsibility.to_string()}</li>
                }).collect_view()}
            </ul>
        </Show>

        </div>
    }
}

#[component]
pub fn Experience(experiences: ReadSignal<Vec<Experience>>) -> impl IntoView {
    create_effect(move |_| {
        logging::log!("experiences = {:?}", experiences.get());
    });
    view! {
        <section id="experience" class="py-16 bg-gradient-to-r from-gray-800 via-gray-900 to-black">
         <h2 class="text-4xl font-bold mb-8 text-center text-white shadow-text font-montserrat">
            EXPERIENCE
         </h2>
         <div class="container mx-auto">
            {move || experiences.get().iter().enumerate().map( |(index, experience)| view! {
                <div key={index} class="mb-12">
                   <div class="flex-column items-start mb-4">
                      <div class="w-20 h-20 bg-white p-1 mr-4 rounded-full shadow-lg">
                         <a
                            href={experience.company_url.clone()}
                            target="_blank"
                            rel="noopener noreferrer">
                            <img
                               src={experience.logo.clone()}
                               alt={format!("{} Logo", experience.company)}
                               class="w-full h-full object-contain rounded-full pointer"
                            />
                         </a>
                      </div>
                      <div>
                         <p class="text-md text-gray-400 shadow-text pt-5 font-montserrat">
                            {experience.location.clone()}
                         </p>
                      </div>
                   </div>
                   {experience.positions.iter().enumerate().map(|(index, position)| view! {
                      <WorkExperienceItem key={index} position={position.clone()} />
                   }).collect_view()}
                </div>
            }).collect_view()
            }
         </div>
      </section>
    }
}
