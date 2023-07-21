use std::{error::Error, io, os::unix::prelude::OsStrExt, path::PathBuf, process::ExitCode};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

mod dir_list;
use dir_list::DriList;

struct App {
    root: PathBuf,
    list: DriList,
    input: String,
    error: String,
}

static RESULT_PATH: &str = "/tmp/vago-result";

fn main() -> Result<ExitCode, Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    match res {
        Ok(code) => Ok(code),
        Err(err) => {
            println!("{err:?}");
            Ok(ExitCode::FAILURE)
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<ExitCode> {
    std::fs::remove_file(RESULT_PATH).ok();

    let root = std::fs::canonicalize(".").unwrap();
    let list = DriList::new(&root);
    let Ok(list) = list else { return Ok(ExitCode::FAILURE) };

    let mut app = App {
        root,
        list,
        input: String::new(),
        error: String::new(),
    };

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q' | 'c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(ExitCode::SUCCESS);
                }

                KeyCode::Up => {
                    app.list.previous();
                }
                KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.list.previous();
                }

                KeyCode::Down => {
                    app.list.next();
                }
                KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.list.next();
                }

                KeyCode::Left => {
                    if let Some(new_root) = app.root.parent() {
                        let new_root = PathBuf::from(new_root);
                        let list = DriList::new(&new_root);

                        match list {
                            Ok(list) => {
                                app.root = new_root;
                                app.list = list;
                                app.input.clear();
                                app.error.clear();
                            }
                            Err(err) => {
                                app.error = err.to_string();
                            }
                        }
                    }
                }

                KeyCode::Esc => {
                    return Ok(ExitCode::SUCCESS);
                }

                KeyCode::Char('y') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    let selected = app.root.as_os_str();
                    std::fs::write(RESULT_PATH, selected.as_bytes()).unwrap();
                    return Ok(ExitCode::from(200));
                }

                KeyCode::Enter | KeyCode::Right => {
                    if let Some(selected) = app.list.selected() {
                        let new_root = PathBuf::from(selected);
                        let list = DriList::new(&new_root);

                        match list {
                            Ok(list) => {
                                app.root = new_root;
                                app.list = list;
                                app.input.clear();
                                app.error.clear();
                            }
                            Err(err) => {
                                app.error = err.to_string();
                            }
                        }
                    }
                }

                KeyCode::Char(ch) => {
                    app.input.push(ch);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }

                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let mut size = f.size();

    let width = (size.width as f32 / 1.5) as u16;
    let height = (size.height as f32 / 1.1) as u16;

    size.x = (size.width - width) / 2;
    size.y = (size.height - height) / 2;

    size.width = width;
    size.height = height;

    let items: Vec<ListItem> = app
        .list
        .fuzzy_match(&app.input)
        .into_iter()
        .map(ListItem::new)
        .collect();

    let items = List::new(items)
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    let error = {
        Paragraph::new(app.error.as_str())
            .style(Style::default().fg(Color::Red))
            .wrap(Wrap { trim: true })
    };

    let input = {
        Paragraph::new(app.input.as_str())
            // .style(Style::default().fg(Color::White).bg(Color::Black))
            .wrap(Wrap { trim: true })
    };

    let block = Block::default()
        // .borders(Borders::ALL)
        .border_type(BorderType::Thick);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(block.inner(size));

    f.render_widget(block, size);
    f.render_widget(error, chunks[0]);
    f.render_widget(input, chunks[1]);
    f.render_stateful_widget(items, chunks[2], app.list.state());
}
