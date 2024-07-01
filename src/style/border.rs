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

fn style_based_selection(hor_style: BorderStyle, vert_style: BorderStyle, ss: &str, ds: &str, sd: &str, dd: &str) -> String {
    match (hor_style, vert_style) {
        (BorderStyle::Single, BorderStyle::Single) => ss,
        (BorderStyle::Double, BorderStyle::Single) => ds,
        (BorderStyle::Single, BorderStyle::Double) => sd,
        (BorderStyle::Double, BorderStyle::Double) => dd,
    }.into()
}

fn get_center_symbol(top: bool, left: bool, right: bool, bottom: bool, hor_style: BorderStyle, vert_style: BorderStyle) -> String {
    match (top, left, right, bottom) {
        // none
        (false, false, false, false) => " ".into(),
        // cross (┼)
        (true, true, true, true) => style_based_selection(hor_style, vert_style, "┼", "╪", "╫", "╬"),
        // top t (┬)
        (false, true, true, true) => style_based_selection(hor_style, vert_style, "┬", "╤", "╥", "╦"),
        // bottom t (┴)
        (true, true, true, false) => style_based_selection(hor_style, vert_style, "┴", "╧", "╨", "╩"),
        // left t (├)
        (true, false, true, true) => style_based_selection(hor_style, vert_style, "├", "╞", "╨", "╟"),
        // right t (┤)
        (true, true, false, true) => style_based_selection(hor_style, vert_style, "┤", "╡", "╢", "╣"),
        // horizontal line (─)
        (true, false, false, true) => if hor_style == BorderStyle::Single { "─" } else { "═" }.into(),
        // vertical line (│)
        (false, true, true, false) => if vert_style == BorderStyle::Single { "│" } else { "║" }.into(),
        // corner (┌)
        (false, false, true, true) => style_based_selection(hor_style, vert_style, "┌", "╒", "╓", "╔"),
        // corner (┐)
        (false, true, false, true) => style_based_selection(hor_style, vert_style, "┐", "╕", "╖", "╗"),
        // corner (└)
        (true, false, true, false) => style_based_selection(hor_style, vert_style, "└", "╘", "╙", "╚"),
        // corner (┘)
        (true, true, false, false) => style_based_selection(hor_style, vert_style, "┘", "╛", "╜", "╝"),
        // single top border
        (true, false, false, false) => if vert_style == BorderStyle::Single { "╵" } else { "║" }.into(),
        // single left border
        (false, true, false, false) => if hor_style == BorderStyle::Single { "╴" } else { "═" }.into(),
        // single right border
        (false, false, true, false) => if hor_style == BorderStyle::Single { "╶" } else { "═" }.into(),
        // single bottom border
        (false, false, false, true) => if vert_style == BorderStyle::Single { "╷" } else { "║" }.into(),
    }
}

pub fn get_common_cell_border_symbol(top_left: Option<&FancyCell>, top_right: Option<&FancyCell>, bottom_left: Option<&FancyCell>, bottom_right: Option<&FancyCell>, hor_style: BorderStyle, vert_style: BorderStyle) -> String {
    let top = match (top_left, top_right) {
        (Some(left), Some(right)) => left.border_style.right.max(right.border_style.left) != BorderLineStyle::None,
        _ => false,
    };

    let left = match (top_left, bottom_right) {
        (Some(top), Some(bot)) => top.border_style.bottom.max(bot.border_style.top) != BorderLineStyle::None,
        _ => false,
    };

    let right = match (top_right, bottom_right) {
        (Some(top), Some(bot)) => top.border_style.bottom.max(bot.border_style.top) != BorderLineStyle::None,
        _ => false,
    };

    let bottom = match (bottom_left, bottom_right) {
        (Some(left), Some(right)) => left.border_style.right.max(right.border_style.left) != BorderLineStyle::None,
        _ => false,
    };

    get_center_symbol(top, left, right, bottom, hor_style, vert_style)
}