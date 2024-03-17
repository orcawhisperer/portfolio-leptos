#[derive(Clone, Debug)]
pub struct TerminalState<'a> {
    pub history: Vec<History<'a>>,
    pub is_typing: bool,
    pub show_terminal: bool,
    pub is_closing: bool,
    pub terminal_show_count: usize,
    pub prank: Prank,
}

impl<'a> TerminalState<'a> {
    pub fn new() -> TerminalState<'a> {
        TerminalState {
            history: Vec::new(),
            is_typing: true,
            show_terminal: true,
            is_closing: false,
            terminal_show_count: 0,
            prank: Prank::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct History<'a> {
    pub command: &'a str,
    pub result: &'a str,
}
impl<'a> History<'a> {
    pub fn new(command: &'a str, result: &'a str) -> History<'a> {
        History { command, result }
    }
}

#[derive(Clone, Debug)]
pub struct Prank {
    pub is_running: bool,
    pub count: usize,
}

impl Prank {
    pub fn new() -> Prank {
        Prank {
            is_running: false,
            count: 0,
        }
    }
}
