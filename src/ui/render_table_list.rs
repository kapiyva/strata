use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, List, ListDirection, ListItem},
    Frame,
};

use crate::model::table::TableName;

pub struct RenderTableListProps<'a> {
    pub table_list: &'a Vec<TableName>,
    pub selected_index: Option<usize>,
    pub focused: bool,
}

pub(super) fn render_table_list(frame: &mut Frame, area: Rect, props: RenderTableListProps) {
    let list_items: Vec<ListItem> = props
        .table_list
        .iter()
        .enumerate()
        .map(|(i, t)| {
            ListItem::new(t.to_string()).style(if Some(i) == props.selected_index {
                Style::default().bg(Color::LightBlue)
            } else {
                Style::default()
            })
        })
        .collect();
    let list = List::new(list_items)
        .block(Block::bordered().title("List"))
        .style(if props.focused {
            Style::default().bold().fg(Color::LightYellow)
        } else {
            Style::default()
        })
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_widget(list, area);
}
