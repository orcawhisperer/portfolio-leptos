use icondata as i;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn NavBar() -> impl IntoView {
    let (is_menu_open, set_is_menu_open) = create_signal(false);

    let links = vec![
        ("About", "#about"),
        ("Experience", "#experience"),
        ("Skills", "#skills"),
        ("Certifications", "#certifications"),
        ("Contact", "#contact"),
    ];

    view! {
        <nav class="bg-gradient-to-r from-gray-800 via-gray-900 to-black shadow-md w-full z-10">
            <div class="container mx-auto px-6 py-3 ">
                <div class="flex items-center justify-between">
                    <div class="text-2xl font-bold text-white font-montserrat">
                        <a href="/">
                            <img src="/assets/images/vk.svg" alt="VK" class="w-20 transition rounded-full duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-105 cursor-pointer" width=100 height=100 />
                        </a>
                    </div>
                    <div class="hidden md:block">
                        <ul class="flex items-center space-x-4">
                            {
                                links.clone()
                                .into_iter().map(|(name, link)| view!{
                                    <li key={name}>
                                        <a
                                            href={link}
                                            class="text-white hover:text-blue-200 uppercase font-montserrat">
                                            {name}
                                        </a>
                                    </li>
                                })
                                .collect_view()
                            }
                        </ul>
                    </div>

                    <div class="md:hidden">
                        <button on:click=move |_| set_is_menu_open.update(|is_menu_open: &mut bool| *is_menu_open = !*is_menu_open)>
                           <Show when=move || is_menu_open.get()>
                                <Icon class="text-[#fff]" icon=i::AiCloseOutlined width="25" height="25"  />
                           </Show>
                           <Show when=move || !is_menu_open.get()>
                                <Icon class="text-[#fff]" icon=i::FaBarsSolid width="25" height="25"  />
                            </Show>
                        </button>
                     </div>

                     <Show when=move || is_menu_open.get()>
                        <div class="mt-4 md:hidden">
                            <ul class="flex flex-col space-y-4">
                                {
                               links.clone().into_iter().map(|(name, link)| view!{
                                     <li key={name}>
                                        <a
                                            href={link}
                                            class="text-white hover:text-blue-200 uppercase font-montserrat">
                                            {name}
                                        </a>
                                    </li>
                                    }).collect_view()
                                }
                            </ul>
                         </div>
                     </Show>

                </div>
            </div>
        </nav>
    }
}
