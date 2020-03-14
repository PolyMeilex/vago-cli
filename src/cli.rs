pub enum Command {
    Add(Option<String>),
    Delete,
    Open,
    Help,
}

pub fn get_command() -> Command {
    if let Some(arg) = std::env::args().nth(1) {
        match arg.as_str() {
            "add" | "-a" => {
                let name = std::env::args().nth(2);
                Command::Add(name)
            }
            "delete" | "-d" => Command::Delete,
            _ => Command::Help,
        }
    } else {
        Command::Open
    }
}
