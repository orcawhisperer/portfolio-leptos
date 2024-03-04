use crate::model::user::Certification;
use leptos::*;

#[component]
pub fn Certifications(certs: ReadSignal<Vec<Certification>>) -> impl IntoView {
    view! {
        <section
        id="certifications"
        class="py-16 bg-gradient-to-r from-gray-800 via-gray-900 to-black">
        <h2 class="text-4xl font-bold mb-8 text-center text-white shadow-text font-montserrat">
           CERTIFICATIONS
        </h2>
        <div class="container mx-auto">
           <div class="flex flex-nowrap overflow-x-auto px-4 gap-4 justify-content-center md:justify-center">
                {
                    move || certs().clone().iter().enumerate().map(|(index, cert)| view!{
                        <div key={index} class="flex flex-col items-center">
                            <a
                               href={cert.link.clone()}
                               target="_blank"
                               rel="noopener noreferrer">
                               <img
                                  src={cert.badge.clone()}
                                  alt={format!("{} badge", cert.title.clone())}
                                  width={200}
                                  height={200}
                                  class="w-42 h-42 mb-4 object-contain rounded transform transition-transform duration-300 hover:scale-105"
                               />
                            </a>
                         </div>
                    }).collect_view()
                }
           </div>
        </div>
     </section>
    }
}
