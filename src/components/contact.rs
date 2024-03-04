use crate::model::user::Contact;
use icondata as i;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn Contact(contacts: ReadSignal<Vec<Contact>>) -> impl IntoView {
    fn render_icon(icon: String) -> View {
        match icon.as_str() {
            "github" => view! {
                <Icon class="text-[#000]" icon=i::BiGithub width="25" height="25"  />
            },
            "twitter" => view! {
                <Icon class="text-[#00ADD8]" icon=i::BiTwitter width="25" height="25"  />
            },
            "linkedin" => view! {
                <Icon class="text-[#0077B5]" icon=i::BiLinkedin width="25" height="25"  />
            },
            "gmail" => view! {
                <Icon class="text-[#EA4335]" icon=i::BiGmail width="25" height="25"  />
            },
            _ => view! {
                <Icon class="text-[#00ADD8]" icon=i::BiTwitter width="25" height="25"  />
            },
        }
        .into_view()
    }

    view! {
        <section
         id="contact"
         class="py-16 bg-gradient-to-r from-gray-800 via-gray-900 to-black">
         <div class="container mx-auto">
            <h2 class="text-4xl font-bold mb-8 text-center text-white shadow-text font-montserrat">
               CONTACT
            </h2>
            <ul class="flex justify-center items-center space-x-8">
            {
                move || contacts().clone().iter().enumerate().map(|(idx, contact)| view!{
                    <a key={idx} href={contact.link.clone()} target="_blank">
                        <div class="text-5xl p-3 rounded-full bg-white shadow-md hover:bg-gray-200 transition-transform duration-300 hover:scale-110">
                           {render_icon(contact.contact_type.clone())}
                        </div>
                    </a>
                }).collect_view()
            }
            </ul>
         </div>
      </section>
    }
}
