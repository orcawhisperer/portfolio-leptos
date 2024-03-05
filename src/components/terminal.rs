use leptos::*;
use wasm_bindgen::UnwrapThrowExt;

#[component]
pub fn Terminal(about_me_text: String) -> impl IntoView {
    let (typed, set_typed) = create_signal(String::new());
    let (current_index, set_current_index) = create_signal(0);
    let typing_speed = 35;
    let about = about_me_text.clone();

    let isTyped = move || (typed().len() == about.clone().len());

    create_effect(move |_| {
        let about = about_me_text.clone();
        let set_typed = set_typed.clone();
        let set_current_index = set_current_index.clone();
        let current_index = current_index.clone();

        let interval_handle = set_interval_with_handle(
            move || {
                let text = about.clone();
                let index = current_index.get();
                if index <= text.len() as i32 {
                    set_typed(text[..index as usize].to_string());
                    set_current_index(index + 1);
                }
            },
            core::time::Duration::from_millis(typing_speed),
        )
        .unwrap_throw();

        on_cleanup(move || {
            interval_handle.clear();
        });
    });

    view! {
        <div class="terminal scroll-smooth bg-black p-6 rounded-md shadow-lg overflow-y-auto flex flex-col h-96 terminal">
            <p class="text-md text-green-500 leading-relaxed md:text-xl lg:text-lg p-2 font-mono">
                {typed.clone()} <Show when=move ||  !isTyped()> <span class="cursor-blink">|</span> </Show>
            </p>
            // <Show when=isTyped.clone()> <input type="text" autofocus="true"></input> </Show>

        </div>
    }
}
