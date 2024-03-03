use leptos::*;

#[component]
pub fn Terminal(about_me_text: String) -> impl IntoView {
    // let (typed, set_typed) = create_signal(String::new());
    // let (current_index, set_current_index) = create_signal(0);

    // create_effect(move |_| {
    //     let about = about_me_text.clone();
    //     let current_index = current_index.clone().get();

    //     spawn_local(async move {
    //         if current_index < about.len() {
    //             logging::log!(
    //                 "current_index = {},  typed = {}",
    //                 current_index.clone(),
    //                 typed().clone()
    //             );
    //             let current_char = about.chars().nth(current_index).unwrap_or(' ');
    //             set_typed.update(|typed: &mut String| typed.push(current_char));
    //             set_current_index.update(|current_index: &mut usize| *current_index += 1);
    //             tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    //         }
    //     })
    // });

    view! {
        <div class="terminal scroll-smooth bg-black p-6 rounded-md shadow-lg overflow-y-auto flex flex-col h-96 terminal">
            <p class="text-md text-green-500 leading-relaxed md:text-xl lg:text-lg p-2 font-mono">
                {move || about_me_text.clone()}
                // <span class="cursor-blink"> | </span>
            </p>
        </div>
    }
}
