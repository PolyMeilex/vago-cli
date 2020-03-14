mod cli;
mod db;
mod help;
mod select;

fn main() {
    let mut db = db::Db::new();

    db.read_file();

    let bkm_labels: Vec<String> = db.data.bookmarks.iter().map(|b| b.as_string()).collect();

    match cli::get_command() {
        cli::Command::Open => {
            if let Some(id) = select::run("Open Dir", &bkm_labels) {
                println!("{}", db.data.bookmarks[id].path);
            }
        }
        cli::Command::Delete => {
            if let Some(id) = select::run("Delete Bookmark", &bkm_labels) {
                if dialoguer::prompts::Confirmation::new()
                    .with_text("Do you want to delete this bookmark?")
                    .interact()
                    .unwrap()
                {
                    db.data.bookmarks.remove(id);
                    db.write_file();
                }
            }
        }
        cli::Command::Add(name) => {
            let path = std::env::var("PWD").unwrap();
            db.add(name, path);
            db.write_file();
        }
        cli::Command::Help => {
            println!("{}", help::HELP);
        }
    }
}
