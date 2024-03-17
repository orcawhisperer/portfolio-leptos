use crate::{
    components::typing_effect::TypingEffect,
    state::terminal::{History, TerminalState},
};
use leptos::*;

#[component]
pub fn Terminal(about_me_text: String) -> impl IntoView {
    let terminal_state: RwSignal<TerminalState<'_>> = expect_context::<RwSignal<TerminalState>>();
    let (input_value, set_input_value) = create_signal(String::new());
    let (_command_history_idx, set_command_history_idx) = create_signal(-1 as i32);

    let (history, set_history) = create_slice(
        terminal_state,
        |terminal_state| terminal_state.history.clone(),
        |terminal_state, history| terminal_state.history = history,
    );

    let (terminal_show_count, _) = create_slice(
        terminal_state,
        |termina_state| termina_state.terminal_show_count,
        |terminal_state, count| terminal_state.terminal_show_count = count,
    );

    let (is_typing, _) = create_slice(
        terminal_state,
        |termina_state| termina_state.is_typing,
        |terminal_state, count| terminal_state.terminal_show_count = count,
    );

    let (show_term, set_show_term) = create_slice(
        terminal_state,
        |termina_state| termina_state.show_terminal,
        |terminal_state, show_term| terminal_state.show_terminal = show_term,
    );

    let _prank_commands = [
        History::new("Executing hack...", "Accessing target system..."),
        History::new("Bypassing firewall...", "Firewall bypassed successfully..."),
        History::new(
            "Cracking password...",
            "Password cracked! Access granted...",
        ),
        History::new(
            "Searching for sensitive data...",
            "Sensitive data found! Downloading...",
        ),
        History::new(
            "Infiltrating network...",
            "Network infiltration successful...",
        ),
        History::new(
            "Disabling security systems...",
            "Security systems disabled...",
        ),
        History::new(
            "Uploading virus...",
            "Virus uploaded! System compromised...",
        ),
        History::new(
            "Gaining control of devices...",
            "Control established! All devices under control...",
        ),
        History::new(
            "Erasing tracks...",
            "Tracks erased! Operation undetected...",
        ),
        History::new(
            "Mission accomplished. Returning control to user...",
            "User control restored. Have a nice day!",
        ),
    ];

    create_effect(move |_| {
        logging::log!("history: {:?}", terminal_state().history);
        logging::log!("input_value: {}", input_value());
        // let h = History::new("ls", "asa");
        // let i = History::new("cat", "asdaasdsadasdas");
        // let j = History::new("ping", "pong!");

        // let mut his = Vec::new();
        // his.push(h);
        // his.push(i);
        // his.push(j);

        // set_history(his);
    });

    fn render_terminal_history(history: &Vec<History>) -> impl IntoView {
        history
            .into_iter()
            .enumerate()
            .map(|(idx, h)| {
                view! {
                    <div key={idx}>
                        <div class="text-sm leading-relaxed font-mono">
                           <span class="text-yellow-400">guest@command-center</span>
                           <span class="text-yellow-500">:-$ </span>
                           <span class="text-green-400">{h.command.to_string()}</span>
                        </div>
                        <div class="text-sm leading-relaxed font-mono">
                           {
                            if idx == history.len() - 1 {
                                view! {
                                    <TypingEffect input_text={h.result.to_string()} />
                                }.into_view()
                            } else {
                                view! {
                                    <span class="text-green-400">{h.result.to_string()}</span>
                                }.into_view()
                            }
                           }
                        </div>
                     </div>
                }
            })
            .collect_view()
    }

    let process_command = move |cmd: String| {
        let mut result = "";
        // let input_val = input_value.clone();
        let input_cmd = cmd.clone().to_lowercase();

        match input_cmd.as_str() {
            "help" | "?" | "h" => result = "try: whoami, contact, skills, exp, cert, clear, exit",
            "hi" | "hello" => result = "Hey there! :)",
            "whoami" => result = "guest",
            "exp" => result = "Engineer@Google [2022 - Present]",
            "skills" => result = "I can juggle with bits and bytes!",
            "contact" => result =
                "Send a pigeon to the nearest tree or drop me a mail me at kvasanth373@gmail.com",
            "cert" => result = "Certified Binary Whisperer!!",
            "clear" => {
                set_input_value("".to_string());
                set_history(Vec::new());
                set_command_history_idx(-1);
            }
            "exit" => {
                set_input_value("".to_string());
                set_history(Vec::new());
                set_show_term(!show_term());
            }
            _ => {
                result = "Invalid Command! Try 'help or h or ?'";
                // set_input_value("Invalid Command!".to_string());
            }
        }
        let mut his = history().clone();

        if result.len() > 0 {
            let h = History::new("", result);
            his.push(h);
            // his.push(History::new(&input_val, result));
            set_history(his);
            set_input_value("".to_string());
            // set_command_history_idx(-1);
        }
    };

    let process_key_down = move |event: String| {
        let input_value = input_value.clone().get(); // Access the value within the RwSignal
                                                     // let history = history.clone().get();
                                                     // let command_history_idx = command_history_idx.clone().get();

        match event.as_str() {
            "Enter" => {
                logging::log!("Enter Key is pressed");
                process_command(input_value);
            }
            "ArrowUp" => {
                logging::log!("Upward Arrow is pressed");
                // if history.len() > 2 {
                //     set_command_history_idx(command_history_idx + 1);
                //     set_input_value(
                //         history[(history.len() as i32 - command_history_idx - 2) as usize]
                //             .command
                //             .to_string(),
                //     );
                // }
            }
            "ArrowDown" => {
                logging::log!("Downward Arrow is pressed");
                // if command_history_idx > 0 {
                //     set_command_history_idx(command_history_idx - 1);
                //     set_input_value(
                //         history[(history.len() as i32 - command_history_idx) as usize]
                //             .command
                //             .to_string(),
                //     );
                // } else if command_history_idx == 0 {
                //     set_command_history_idx(-1);
                //     set_input_value("".to_string());
                // }
            }
            _ => {}
        };
    };

    fn handle_up_arrow() {}

    view! {
        <div class="terminal scroll-smooth bg-black p-6 rounded-md shadow-lg overflow-y-auto flex flex-col h-96 terminal">
            <div class="relative flex-grow">
                <Show when=move || terminal_show_count() < 1>
                    <span class="text-sm leading-relaxed font-mono">
                        <span class="text-red-400">
                            vasanth@command-center
                        </span>
                        <span class="text-red-500">:/# </span>
                        </span>
                        <TypingEffect input_text={about_me_text.clone()}  />
                </Show>
                {move || render_terminal_history(&history())}
                <Show when=move || !is_typing()>
                    <div class="flex w-full">
                        <span class="text-sm leading-relaxed font-mono">
                           <span class="text-yellow-400">guest@command-center</span>
                           <span class="text-yellow-500">:-$ </span>
                        </span>

                        <div class="input-container flex-grow relative pl-2">
                           <input
                              type="text"
                              value=move || input_value()
                              on:input=move|ev|{
                                set_input_value(event_target_value(&ev));
                              }
                              on:keydown=move|ev|{
                                process_key_down(ev.key());
                              }
                            //   onKeyDown={handleKeyDown()}
                              class="block bg-transparent text-green-500 font-mono focus:outline-none flex-grow w-full truncate"
                              style="caret-color: green;"
                              autofocus
                           />
                        </div>
                     </div>
                </Show>
             </div>
        </div>
    }
}
