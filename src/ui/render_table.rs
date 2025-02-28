use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::model::table::TableData;

pub struct RenderTableProps<'a> {
    pub table: &'a TableData,
    pub focused: bool,
}

pub(super) fn render_table(frame: &mut Frame, area: Rect, props: RenderTableProps) {
    let table = props.table;
    let (selected_row, selected_col) = props
        .table
        .get_selector_index()
        .map(|(row, col)| (Some(row), Some(col)))
        .unwrap_or((None, None));

    let header_style = |col: usize| -> Style {
        if Some(col) == selected_col {
            return Style::default().bold().bg(Color::LightBlue);
        };
        Style::default().bold()
    };

    let header = Row::new(
        std::iter::once(Cell::from("#")).chain(
            table
                .headers
                .iter()
                .enumerate()
                .map(|(col, header)| Cell::from(header.clone()).style(header_style(col))),
        ),
    )
    .bottom_margin(1);

    let index_style = |row: usize| -> Style {
        if Some(row) == selected_row {
            return Style::default().bg(Color::LightBlue);
        };
        Style::default()
    };
    let cell_style = |(row, col): (usize, usize)| -> Style {
        if Some((row, col)) == props.table.get_selector_index() {
            return Style::default().bg(Color::LightBlue);
        };
        Style::default()
    };

    let body = table.rows.iter().enumerate().map(|(row_index, row)| {
        Row::new(
            std::iter::once(Cell::from(row_index.to_string()).style(index_style(row_index))).chain(
                row.iter().enumerate().map(|(col_index, cell_value)| {
                    Cell::from(cell_value.clone()).style(cell_style((row_index, col_index)))
                }),
            ),
        )
    });

    let table = Table::new(body, get_header_widths(&table))
        .block(Block::default().title("Table").borders(Borders::ALL))
        .style(if props.focused {
            Style::default().bold().fg(Color::LightYellow)
        } else {
            Style::default()
        })
        .header(header);

    frame.render_stateful_widget(table, area, &mut props.table.table_view_state.clone());
}

/// Get the widths of each header, which is maximum width of the header and column values
fn get_header_widths(table: &TableData) -> Vec<Constraint> {
    vec![Constraint::Length(3)]
        .into_iter()
        .chain(table.headers.iter().enumerate().map(|(index, header)| {
            let max_row_width = table
                .rows
                .iter()
                .map(|row| row.get(index).map(String::len).unwrap_or(0))
                .max()
                .unwrap_or(0);
            Constraint::Length(max_row_width.max(header.len()) as u16)
        }))
        .collect()
}

#[cfg(test)]
mod tests {
    use ratatui::widgets::TableState;

    use super::*;
    use crate::model::table::TableData;

    #[test]
    fn test_get_header_widths() {
        let table = TableData {
            exist_headers: true,
            headers: vec!["header1".to_string(), "header2".to_string()],
            rows: vec![
                vec!["row1col1".to_string(), "row1col2".to_string()],
                vec!["row2col1".to_string(), "row2col2".to_string()],
            ],
            table_view_state: TableState::default().with_selected_cell(Some((0, 0))),
        };
        let widths = get_header_widths(&table);
        assert_eq!(
            widths,
            vec![
                Constraint::Length(3),
                Constraint::Length(8),
                Constraint::Length(8)
            ]
        );
    }
}
