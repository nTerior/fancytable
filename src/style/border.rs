use crate::{FancyCell, FancyTable};

/// The thickness of a border row/column.
/// Applies to the entire drawn line.
///
/// Using [BorderStyle::Double] leads to only [BorderLineStyle::Dashed] and [BorderLineStyle::Dotted] being ignored,
/// the line will always be solid
#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub enum BorderStyle {
    #[default]
    Single,
    Double,
}

/// The styles for the outline of the table
#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub struct TableOutline {
    pub top: BorderStyle,
    pub left: BorderStyle,
    pub right: BorderStyle,
    pub bottom: BorderStyle,
}

/// The line style.
/// Only applies if [BorderStyle::Single] is being used
///
/// [BorderLineStyle::None] beats [BorderLineStyle::Dotted] beats [BorderLineStyle::Dashed] beats [BorderLineStyle::Solid]
/// when choosing a line and adjacent cells try setting their separating border style
///
/// Setting the outline border style of the whole table has no effect
#[derive(Debug, Eq, PartialEq, Copy, Clone, Default, Ord, PartialOrd)]
#[repr(u8)]
pub enum BorderLineStyle {
    #[default]
    Solid = 0,
    Dashed = 1,
    Dotted = 2,
    None = 3,
}

/// The line styles for a single cell
#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub struct CellBorderStyle {
    pub top: BorderLineStyle,
    pub left: BorderLineStyle,
    pub right: BorderLineStyle,
    pub bottom: BorderLineStyle,
}

/// Returns the vertical cell-cell separator.
/// If there is no cell at (row_idx, col_idx), this function panics.
/// Also, if the selected cell has no right neighbour it's right [CellBorderStyle] will be ignored
pub fn get_right_cell_separator(table: &FancyTable, row_idx: usize, col_idx: usize) -> String {
    let cell = table.get(row_idx, col_idx)
        .expect(format!("There is no cell at ({},{})", row_idx, col_idx).as_str());
    // right neighbour
    let adjacent = table.get(row_idx, col_idx + 1);

    get_border_symbol(cell,
                      adjacent,
                      table.outline.right,
                      table.get_vertical_separator_style(col_idx),
                      |cell, adj| cell.border_style.right.max(adj.border_style.left),
                      ["│", "║", "╵", "┆"].map(String::from))
}


/// Returns the horizontal cell-cell separator.
/// If there is no cell at (row_idx, col_idx), this function panics.
/// Also, if the selected cell has no bottom neighbour it's bottom [CellBorderStyle] will be ignored
pub fn get_bottom_cell_separator(table: &FancyTable, row_idx: usize, col_idx: usize) -> String {
    let cell = table.get(row_idx, col_idx)
        .expect(format!("There is no cell at ({},{})", row_idx, col_idx).as_str());
    // right neighbour
    let adjacent = table.get(row_idx + 1, col_idx);

    get_border_symbol(cell,
                      adjacent,
                      table.outline.bottom,
                      table.get_horizontal_separator_style(col_idx),
                      |cell, adj| cell.border_style.bottom.max(adj.border_style.top),
                      ["─", "═", "╴", "┄"].map(String::from))
}

/// Usage: [single solid, double solid, single dashed, single dotted]
type Charset = [String; 4];

fn get_border_symbol(cell: &FancyCell, adjacent: Option<&FancyCell>, table_outline: BorderStyle, table_line_style: Option<&BorderStyle>, get_line_style: fn(&FancyCell, &FancyCell) -> BorderLineStyle, charset: Charset) -> String {
    if adjacent.is_none() {
        return match table_outline {
            BorderStyle::Single => charset[0].clone(),
            BorderStyle::Double => charset[1].clone(),
        };
    }

    let line_style = get_line_style(cell, adjacent.unwrap());
    match (line_style, table_line_style.unwrap()) {
        (BorderLineStyle::Solid, BorderStyle::Single) => charset[0].clone(),
        (BorderLineStyle::Dashed, BorderStyle::Single) => charset[2].clone(),
        (BorderLineStyle::Dotted, BorderStyle::Single) => charset[3].clone(),
        (BorderLineStyle::None, _) => " ".into(),
        (_, BorderStyle::Double) => charset[1].clone(),
    }
}