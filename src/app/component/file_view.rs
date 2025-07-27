use std::{fs, path::PathBuf};

use eyre::Result;
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::component::{component_style, selectable_item_style_factory, StrataComponent};

pub struct FileView {
    state: ListState,
    items: Vec<(PathBuf, String)>,
    current_path: PathBuf,
}

impl FileView {
    pub fn new() -> Result<Self> {
        let current_path = std::env::current_dir()?;
        let items = Self::load_directory_items(&current_path)?;
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }
        
        Ok(Self {
            state,
            items,
            current_path,
        })
    }

    fn load_directory_items(path: &PathBuf) -> Result<Vec<(PathBuf, String)>> {
        let mut items = Vec::new();
        
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                let name = entry_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("<invalid>")
                    .to_string();
                
                let display_name = if entry_path.is_dir() {
                    format!("📁 {}", name)
                } else {
                    format!("📄 {}", name)
                };
                
                items.push((entry_path, display_name));
            }
        }
        
        // Sort directories first, then files
        items.sort_by(|(a, _), (b, _)| {
            match (a.is_dir(), b.is_dir()) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.cmp(b),
            }
        });
        
        Ok(items)
    }

    pub fn expand_directory(&mut self) -> Result<&mut Self> {
        if let Some(selected_index) = self.state.selected() {
            if let Some((path, _)) = self.items.get(selected_index) {
                if path.is_dir() {
                    self.current_path = path.clone();
                    self.items = Self::load_directory_items(&self.current_path)?;
                    self.state.select(if self.items.is_empty() { None } else { Some(0) });
                }
            }
        }
        Ok(self)
    }

    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        
        let selected = self.state.selected().unwrap_or(0);
        let next = if selected >= self.items.len() - 1 {
            0
        } else {
            selected + 1
        };
        self.state.select(Some(next));
    }

    pub fn select_prev(&mut self) {
        if self.items.is_empty() {
            return;
        }
        
        let selected = self.state.selected().unwrap_or(0);
        let prev = if selected == 0 {
            self.items.len() - 1
        } else {
            selected - 1
        };
        self.state.select(Some(prev));
    }

    pub fn selected_path(&self) -> Option<PathBuf> {
        if let Some(selected_index) = self.state.selected() {
            self.items.get(selected_index).map(|(path, _)| path.clone())
        } else {
            None
        }
    }

    pub fn go_up_directory(&mut self) -> Result<&mut Self> {
        if let Some(parent) = self.current_path.parent() {
            self.current_path = parent.to_path_buf();
            self.items = Self::load_directory_items(&self.current_path)?;
            self.state.select(if self.items.is_empty() { None } else { Some(0) });
        }
        Ok(self)
    }
}

impl StrataComponent for FileView {
    fn render(&self, frame: &mut Frame, area: Rect, is_focused: bool) {
        let block = Block::default()
            .title(format!("Files - {}", self.current_path.display()))
            .borders(Borders::ALL)
            .border_style(component_style(is_focused));

        let items: Vec<ListItem> = self.items
            .iter()
            .map(|(_, display_name)| ListItem::new(display_name.as_str()))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(selectable_item_style_factory(is_focused)(true));

        frame.render_stateful_widget(list, area, &mut self.state.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_view() {
        let file_view = FileView::new().unwrap();
        assert!(file_view.current_path.exists());
    }

    #[test]
    fn test_expand_directory() {
        let mut file_view = FileView::new().unwrap();
        if !file_view.items.is_empty() {
            file_view.select_next();
            let result = file_view.expand_directory();
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_navigation() {
        let mut file_view = FileView::new().unwrap();
        if !file_view.items.is_empty() {
            let initial = file_view.state.selected();
            file_view.select_next();
            let after_next = file_view.state.selected();
            
            if file_view.items.len() > 1 {
                assert_ne!(initial, after_next);
            }
            
            file_view.select_prev();
            assert_eq!(file_view.state.selected(), initial);
        }
    }

    #[test]
    fn test_go_up_directory() {
        let mut file_view = FileView::new().unwrap();
        let original_path = file_view.current_path.clone();
        
        let result = file_view.go_up_directory();
        assert!(result.is_ok());
        
        // If we're not at root, path should have changed
        if original_path.parent().is_some() {
            assert_ne!(file_view.current_path, original_path);
        }
    }
}