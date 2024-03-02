use leptos::*;

#[component]
pub fn NavBar() -> impl IntoView {
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
                        <a href="/"> {"HOME"} </a>
                    </div>
                    <div class="hidden md:block">
                        <ul class="flex items-center space-x-4">
                            {
                                links
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
                </div>
            </div>
        </nav>
    }
}
