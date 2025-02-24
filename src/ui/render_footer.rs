use eyre::Result;
use ratatui::{layout::Rect, text::Line, Frame};

use crate::model::app::App;

// enum OperationGuide {
//     AddTable,
//     SelectTable,
//     SelectCell,
//     EditHeader,
//     EditCell,
// }

pub(super) fn render_footer(frame: &mut Frame, area: Rect, app: &App) -> Result<()> {
    // let operation_guide = match app.get_display_mode() {
    //     DisplayMode::AddTable => OperationGuide::AddTable,
    //     DisplayMode::SelectTable => OperationGuide::SelectTable,
    //     DisplayMode::SelectCell => OperationGuide::SelectCell,
    //     DisplayMode::EditHeader => OperationGuide::EditHeader,
    //     DisplayMode::EditCell => OperationGuide::EditCell,
    // };
    // let footer = Line::from(app.get_display_mode().to_string());
    let footer = Line::from(format!(
        "{:?}, {:?}, {:?}, {:?}",
        app.get_display_mode(),
        app.get_selected_table_name(),
        app.get_selected_cell(),
        app.get_table_state()
    ));
    frame.render_widget(footer, area);
    Ok(())
}
