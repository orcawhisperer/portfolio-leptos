use leptos::*;

#[component]
pub fn Terminal(about_me_text: String) -> impl IntoView {
    view! {
        <div class="terminal scroll-smooth bg-black p-6 rounded-md shadow-lg overflow-y-auto flex flex-col h-96">
            <p class="text-md text-green-500 leading-relaxed md:text-xl lg:text-lg p-2 font-mono">
                {about_me_text}
            </p>
        </div>
    }
}
