use leptos::*;

#[component]
pub fn Hero(name: ReadSignal<String>, title: ReadSignal<String>) -> impl IntoView {
    create_effect(move |_| {
        logging::log!("name = {} title = {}", name().clone(), title().clone());
    });

    view! {
        <section
        id="contact"
        class="py-16 bg-gradient-to-r from-gray-800 via-gray-900 to-black">
        <div class="container mx-auto">
           <h2 class="text-4xl  mb-8 text-center text-white shadow-text font-montserrat">
              {move || name().clone()}
           </h2>
           <h3 class="text-xl mb-8 text-center text-white font-montserrat italic">
            {move || title().clone()}
           </h3>
        </div>
      </section>
    }
}
