use leptos::*;
use wasm_bindgen::UnwrapThrowExt;

use crate::state::terminal::TerminalState;

#[component]
pub fn TypingEffect(input_text: String) -> impl IntoView {
    let (typed, set_typed) = create_signal(String::new());
    let (current_index, set_current_index) = create_signal(0 as usize);
    let typing_speed = 35;

    let terminal_state = expect_context::<RwSignal<TerminalState>>();

    let (is_typing, set_is_typing) = create_slice(
        terminal_state,
        |terminal_state| terminal_state.is_typing,
        |terminal_state, is_tyiping| terminal_state.is_typing = is_tyiping,
    );

    create_effect(move |_| {
        let about = input_text.clone();
        let set_typed = set_typed.clone();
        let set_current_index = set_current_index.clone();
        let current_index = current_index.clone();
        let interval_handle = set_interval_with_handle(
            move || {
                let text = about.clone();
                let index = current_index.get();
                if index <= text.len() {
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

        if typed().len() == input_text.len() {
            set_is_typing(false);
        } else {
            set_is_typing(true);
        }
    });

    view! {
        <span class="text-green-400 text-sm leading-relaxed font-mono">
            {typed.clone()} <Show when=move ||  is_typing().clone()> <span class="cursor-blink">|</span> </Show>
        </span>
    }
}
