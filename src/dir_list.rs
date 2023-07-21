use std::{io, path::Path};

use ratatui::widgets::ListState;

pub struct DriList {
    state: ListState,
    dirs: Vec<String>,
    items: Vec<String>,
}

impl DriList {
    pub fn new(start_dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir = std::fs::read_dir(start_dir.as_ref())?;

        let dir = dir
            .into_iter()
            .filter_map(|file| file.ok())
            .filter(|file| file.file_type().map(|ty| ty.is_dir()).unwrap_or(false))
            .map(|dir| dir.path())
            .filter(|path| {
                path.file_name()
                    .map(|name| !name.to_string_lossy().starts_with('.'))
                    .unwrap_or(true)
            })
            .map(|path| path.to_string_lossy().to_string())
            .collect();

        Ok(DriList::with_items(dir))
    }

    fn set_items(&mut self, items: Vec<String>) {
        self.items = items;

        if self.items.is_empty() {
            self.state.select(None);
        }
    }

    pub fn state(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn fuzzy_match(&mut self, input: &str) -> Vec<String> {
        if input.is_empty() {
            self.set_items(self.dirs.clone());
            return self.items.clone();
        }

        let mut items: Vec<_> = self
            .dirs
            .iter()
            .filter_map(|item| sublime_fuzzy::best_match(input, item).map(|res| (item, res)))
            .collect();

        items.sort_by(|(_, a), (_, b)| b.score().cmp(&a.score()));

        let items = items.into_iter().map(|(item, _)| item.clone()).collect();
        self.set_items(items);

        self.items.clone()
    }

    fn with_items(items: Vec<String>) -> DriList {
        let mut state = ListState::default();
        state.select(Some(0));
        DriList {
            state,
            dirs: items.clone(),
            items,
        }
    }

    pub fn selected(&self) -> Option<&str> {
        self.state.selected().map(|id| self.items[id].as_str())
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
