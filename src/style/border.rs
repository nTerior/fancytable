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

/// The line style.
/// Only applies if [BorderStyle::Single] is being used
///
/// [BorderLineStyle::None] beats [BorderLineStyle::Dotted] beats [BorderLineStyle::Dashed] beats [BorderLineStyle::Solid]
/// when choosing a line style and between adjacent cells
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

fn get_horizontal_symbol(line: &BorderLineStyle, style: &BorderStyle) -> String {
    match (line, style) {
        (BorderLineStyle::Solid, BorderStyle::Single) => "─",
        (BorderLineStyle::Dashed, BorderStyle::Single) => "╴",
        (BorderLineStyle::Dotted, BorderStyle::Single) => "┄",
        (BorderLineStyle::None, _) => " ",
        (_, BorderStyle::Double) => "═",
    }.to_string()
}

fn get_vertical_symbol(line: &BorderLineStyle, style: &BorderStyle) -> String {
    match (line, style) {
        (BorderLineStyle::Solid, BorderStyle::Single) => "│",
        (BorderLineStyle::Dashed, BorderStyle::Single) => "╵",
        (BorderLineStyle::Dotted, BorderStyle::Single) => "┆",
        (BorderLineStyle::None, _) => " ",
        (_, BorderStyle::Double) => "║",
    }.to_string()
}

pub fn get_cell_border_symbols(table: &FancyTable, cell_row: usize, cell_col: usize) -> (String, String, String, String) {
    let cell = table.get(cell_row, cell_col).unwrap_or(&FancyCell::default());
    let top = match cell_row - 1 {
        i if i < 0 => None,
        i => table.get(i, cell_col),
    }.unwrap_or(&FancyCell::default());
    let left = match cell_col - 1 {
        i if i < 0 => None,
        i => table.get(cell_row, i),
    }.unwrap_or(&FancyCell::default());
    let right = table.get(cell_row, cell_col + 1).unwrap_or(&FancyCell::default());
    let bottom = table.get(cell_row + 1, cell_col).unwrap_or(&FancyCell::default());

    let top_hor_style = table.get_horizontal_separator_style(cell_row);
    let bottom_hor_style = table.get_horizontal_separator_style(cell_row + 1);
    let left_vert_style = table.get_vertical_separator_style(cell_col);
    let right_vert_style = table.get_vertical_separator_style(cell_col + 1);

    let top_symbol = get_horizontal_symbol(&cell.border_style.top.max(top.border_style.bottom), top_hor_style);
    let bottom_symbol = get_horizontal_symbol(&cell.border_style.bottom.max(bottom.border_style.top), bottom_hor_style);
    let left_symbol = get_vertical_symbol(&cell.border_style.left.max(left.border_style.right), left_vert_style);
    let right_symbol = get_vertical_symbol(&cell.border_style.right.max(right.border_style.left), right_vert_style);

    (top_symbol, left_symbol, right_symbol, bottom_symbol)
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