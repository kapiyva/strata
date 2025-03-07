use eyre::{bail, Result};
use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, List, ListDirection, ListItem},
};

use crate::error::StrataError;

use super::{component_style, selectable_item_style_factory, StrataComponent};

pub const INITIAL_TABLE_NAME: &str = "new_table";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TableName(String);

impl TableName {
    pub fn from<T>(_name: T) -> Result<TableName>
    where
        T: ToString,
    {
        let name = _name.to_string();
        if name.is_empty() {
            bail!(StrataError::InvalidTableName)
        } else {
            Ok(Self(name))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ToString for TableName {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Default, Clone)]
pub struct TableSelector {
    table_list: Vec<TableName>,
    selected: Option<usize>,
}

impl TableSelector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(table_list: Vec<TableName>) -> Self {
        Self {
            table_list,
            selected: Some(0),
        }
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.selected
    }

    pub fn selected_index_by_name(&self, table_name: &TableName) -> Option<usize> {
        self.table_list.iter().position(|t| t == table_name)
    }

    pub fn selected_table_name(&self) -> Option<&TableName> {
        self.selected.map(|i| &self.table_list[i])
    }

    pub fn is_empty(&self) -> bool {
        self.table_list.is_empty()
    }

    /// Add new table to list and select it
    pub fn push_table(&mut self, table: TableName) -> Result<&mut Self> {
        if self.table_list.contains(&table) {
            bail!(StrataError::TableNameDuplicate(table.to_string()));
        }

        self.table_list.push(table);
        self.selected = Some(self.table_list.len().saturating_sub(1));

        Ok(self)
    }

    pub fn remove_table(&mut self, remove_index: usize) -> Result<&mut Self> {
        if remove_index > self.table_list.len() {
            bail!(StrataError::IndexOutOfBounds {
                max: self.table_list.len(),
                requested: remove_index,
            });
        }

        self.table_list.remove(remove_index);
        self.selected = if self.table_list.is_empty() {
            None
        } else {
            self.selected_index()
                .map(|i| i.min(self.table_list.len() - 1))
        };
        Ok(self)
    }

    pub fn update_table(&mut self, index: usize, table: TableName) -> Result<&mut Self> {
        if index > self.table_list.len() {
            bail!(StrataError::IndexOutOfBounds {
                max: self.table_list.len(),
                requested: index,
            });
        }

        self.table_list[index] = table;
        Ok(self)
    }

    pub fn select_next(&mut self) -> &mut Self {
        if self.table_list.is_empty() {
            self.selected = None;
            return self;
        }

        self.selected = self
            .selected
            .map(|i| i.saturating_add(1).min(self.table_list.len() - 1))
            .or(Some(0));
        self
    }

    pub fn select_prev(&mut self) -> &mut Self {
        if self.table_list.is_empty() {
            self.selected = None;
            return self;
        }

        self.selected = self.selected.map(|i| i.saturating_sub(1)).or(Some(0));
        self
    }

    pub fn select_by_name(&mut self, table_name: &TableName) -> Result<&mut Self> {
        self.selected = self.selected_index_by_name(table_name);

        if self.selected.is_none() {
            bail!(StrataError::TableNotFound(table_name.to_string()));
        }

        Ok(self)
    }
}

impl StrataComponent for TableSelector {
    fn render(&self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect, is_focused: bool) {
        let item_style = selectable_item_style_factory(is_focused);

        let list_items: Vec<ListItem> = self
            .table_list
            .iter()
            .enumerate()
            .map(|(i, t)| ListItem::new(t.to_string()).style(item_style(Some(i) == self.selected)))
            .collect();
        let list = List::new(list_items)
            .block(Block::bordered().title("List"))
            .style(component_style(is_focused))
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        frame.render_widget(list, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl TableSelector {
        pub fn get_table_list(&self) -> &Vec<TableName> {
            &self.table_list
        }
    }

    pub fn setup() -> TableSelector {
        TableSelector::from(vec![
            TableName::from("table1").unwrap(),
            TableName::from("table2").unwrap(),
        ])
    }

    #[test]
    fn test_get_index() {
        let sl = setup();

        assert_eq!(sl.selected_index().unwrap(), 0);
        assert_eq!(
            sl.selected_index_by_name(&TableName::from("table1").unwrap()),
            Some(0)
        );
        assert_eq!(
            sl.selected_index_by_name(&TableName::from("table2").unwrap()),
            Some(1)
        );
    }

    #[test]
    fn test_move_selector() {
        let mut sl = setup();

        // check down table selector
        let sl = sl.select_next();
        assert_eq!(sl.selected_index(), Some(1),);
        // check out of bound
        let sl = sl.select_next();
        assert_eq!(sl.selected_index(), Some(1),);

        // check up table selector
        let sl = sl.select_prev();
        assert_eq!(sl.selected_index(), Some(0),);
        // check out of bound
        let sl = sl.select_prev();
        assert_eq!(sl.selected_index(), Some(0),);
    }

    #[test]
    fn test_add_table() {
        let mut sl = setup();

        sl.push_table(TableName::from("table3").unwrap()).unwrap();
        assert_eq!(sl.get_table_list().len(), 3);
        assert_eq!(sl.selected_index(), Some(2));

        sl.push_table(TableName::from("table4").unwrap()).unwrap();
        assert_eq!(sl.get_table_list().len(), 4);
        assert_eq!(sl.selected_index(), Some(3));
    }

    #[test]
    fn test_remove_table() {
        let mut sl = setup();
        sl.selected = Some(1);

        let sl = sl.remove_table(1).unwrap();
        assert_eq!(sl.get_table_list().len(), 1);
        assert_eq!(sl.selected_index(), Some(0));

        let sl = sl.remove_table(0).unwrap();
        assert_eq!(sl.get_table_list().len(), 0);
        assert_eq!(sl.selected_index(), None);
    }
}
