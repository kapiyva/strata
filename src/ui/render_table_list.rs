use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, List, ListDirection, ListItem},
    Frame,
};

use crate::model::app::App;

pub(super) fn render_table_list(
    frame: &mut Frame,
    area: Rect,
    // table_list: &Vec<TableName>,
    app: &App,
) {
    let list_items: Vec<ListItem> = app
        .get_table_list()
        .iter()
        .map(|t| {
            ListItem::new(t.to_string()).style(if Some(t) == app.get_selected_table_name() {
                Style::default().bg(Color::LightBlue)
            } else {
                Style::default()
            })
        })
        .collect();
    let list = List::new(list_items)
        .block(Block::bordered().title("List"))
        // .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_widget(list, area);
}
