use std::cmp::max;
use std::fmt::{Display, Formatter};
use crate::FancyCell;
use crate::style::border::{BorderStyle, get_cell_border_symbols, get_common_cell_border_symbol};

/// A stylizable, rectangular table for pretty cli output.
#[derive(Debug, Eq, PartialEq, Default)]
pub struct FancyTable {
    /// Access: `cells[row][col]`
    cells: Vec<Vec<FancyCell>>,
    /// Set when adding a column to an empty table, so that a call on [FancyTable::add_rows] creates the correct result
    /// ONLY FOR INTERNAL USE!
    _added_column_first: bool,
    /// The vertical separators + borders
    vertical_separator_styles: Vec<BorderStyle>,
    /// The horizontal separators + border
    horizontal_separator_styles: Vec<BorderStyle>,
}

impl FancyTable {
    /// Creates a new table from a 2d-field of string
    /// The strings will be converted to [FancyCell]s
    /// Also, every row will have the same amount of columns after initialization
    /// # Example
    /// ```
    /// use fancytable::FancyTable;
    /// let table = FancyTable::new(vec![
    ///     vec!["Hello".into(), "World".into()],
    ///     vec!["Lorem".into(), "Ipsum".into(), "dolor".into()],
    /// ]);
    /// ```
    pub fn new(content: Vec<Vec<String>>) -> FancyTable {
        // converts the strings into cells
        let mut cells: Vec<Vec<FancyCell>> = content.iter()
            .map(|row| row.iter()
                .map(String::from)
                .map(FancyCell::from)
                .collect::<Vec<FancyCell>>())
            .collect();

        // gets the maximum number of columns in all rows
        let columns = cells.iter()
            .map(|row| row.len())
            .max()
            .unwrap_or(0);

        // fills every row to the maximum number of columns
        // therefore, the table is now a rectangle
        for row in &mut cells {
            if row.len() < columns {
                for _ in 0..(columns - row.len()) {
                    row.push(FancyCell::default());
                }
            }
        }

        let vertical_separators: usize = max(columns + 1, 2);
        let horizontal_separators: usize = max(cells.len() + 1, 2);

        FancyTable {
            vertical_separator_styles: vec![BorderStyle::default(); vertical_separators],
            horizontal_separator_styles: vec![BorderStyle::default(); horizontal_separators],
            _added_column_first: false,
            cells,
        }
    }

    /// Adds a number of rows.
    /// The rows will be filled with default [FancyCell]s
    /// The amount of columns stays the same
    pub fn add_rows(&mut self, n: usize) {
        let mut rows = n;

        // Check if a column has been added first and adds 1 row less
        // since a row has already been added by the addition of a column
        if self._added_column_first {
            self._added_column_first = false;
            rows -= 1;
        }

        let cols = self.cells.get(0).unwrap_or(&vec![].into()).len();
        for _ in 0..rows {
            self.cells.push(vec![FancyCell::default(); cols]);
            self.horizontal_separator_styles.push(BorderStyle::default());
        }
    }

    /// Adds a number of columns.
    /// The columns will be filled with default [FancyCell]s
    ///
    /// # Important
    /// If this function is called AND the table is empty, an empty row will be added.
    /// This row will then be filled with the amount of columns requested.
    /// BUT calling [FancyTable::add_rows] THE FIRST TIME will create ONE ROW LESS,
    /// so that the expected geometry will be created
    ///
    /// # Example
    /// ```
    /// use fancytable::FancyTable;
    /// let mut table = FancyTable::default();
    /// table.add_columns(2);
    /// table.add_rows(2);
    /// // this will result in a 2x2 table
    /// ```
    pub fn add_columns(&mut self, n: usize) {
        if self.cells.len() == 0 {
            self.cells.push(vec![]);
            self._added_column_first = true;
        }

        for _ in 0..n {
            for row in &mut self.cells {
                row.push(FancyCell::default());
            }
            self.vertical_separator_styles.push(BorderStyle::default());
        }
    }

    /// Sets the cell at a specified position starting at (0, 0)
    /// Will create rows and columns dynamically if needed.
    ///
    /// Returns whether rows or columns have been created
    ///
    /// # Example
    /// ```
    /// use fancytable::FancyTable;
    /// let mut table = FancyTable::default(); // Empty table
    /// table.set(5, 5, "Hello World".into()); // creates 6 rows and 6 columns
    /// ```
    pub fn set(&mut self, row_idx: usize, col_idx: usize, cell: FancyCell) -> bool {
        let mut edited = false;
        if row_idx >= self.cells.len() {
            self.add_rows(row_idx - self.cells.len() + 1);
            edited = true;
        }
        if col_idx >= self.cells[row_idx].len() {
            self.add_columns(col_idx - self.cells[row_idx].len() + 1);
            edited = true;
        }

        self.cells[row_idx][col_idx] = cell;
        edited
    }

    /// Returns a reference to the [FancyCell] at the position (row_idx, col_idx)
    /// Returns [None] if not found
    pub fn get(&self, row_idx: usize, col_idx: usize) -> Option<&FancyCell> {
        let row = self.cells.get(row_idx)?;
        row.get(col_idx)
    }

    /// Returns a reference to the [FancyCell] at the(row_idx, col_idx) or [None] if not found
    /// Returns [None] if any variable is negative
    pub fn get_cell(&self, row: i64, col: i64) -> Option<&FancyCell> {
        if row < 0 || col < 0 {
            None
        } else {
            self.get(row as usize, col as usize)
        }
    }

    /// Returns the maximum height of a given row
    pub fn get_row_height(&self, row_idx: usize) -> usize {
        self.cells[row_idx].iter()
            .map(|cell| cell.get_height())
            .max()
            .unwrap_or(0)
    }

    /// Returns a mutable reference to the [FancyCell] at the position (row_idx, col_idx) in the table
    /// Returns None if not found
    pub fn get_mut(&mut self, row_idx: usize, col_idx: usize) -> Option<&mut FancyCell> {
        let row = self.cells.get_mut(row_idx)?;
        row.get_mut(col_idx)
    }

    /// Returns the amount of rows currently in the table
    pub fn get_row_count(&self) -> usize {
        self.cells.len()
    }

    /// Returns the amount of columns currently in the table
    pub fn get_column_count(&self) -> usize {
        if self.cells.len() != 0 {
            // since the table is always rectangular, this will always work
            return self.cells[0].len();
        }
        0
    }

    /// Returns the style for a single vertical separator (not the outline)
    pub fn get_vertical_separator_style(&self, idx: usize) -> Option<&BorderStyle> {
        self.vertical_separator_styles.get(idx)
    }

    /// Returns the style for a single horizontal separator (not the outline)
    pub fn get_horizontal_separator_style(&self, idx: usize) -> Option<&BorderStyle> {
        self.horizontal_separator_styles.get(idx)
    }

    /// Sets the style for a vertical separator (not the outline).
    pub fn set_vertical_separator_style(&mut self, idx: usize, style: BorderStyle) {
        self.vertical_separator_styles[idx] = style;
    }

    /// Sets the style for a horizontal separator (not the outline).
    pub fn set_horizontal_separator_style(&mut self, idx: usize, style: BorderStyle) {
        self.horizontal_separator_styles[idx] = style;
    }
}

impl FancyTable {
    fn get_col_widths(&self) -> Vec<usize> {
        let columns = self.get_column_count();
        let mut widths = Vec::with_capacity(columns);

        for i in 0..columns {
            let width = self.cells.iter()
                .map(|row| row[i].get_width())
                .max()
                .unwrap_or(0);
            widths.push(width);
        }

        widths
    }

    /// Writes the top border of a single row to the formatter
    fn write_top_border(&self, f: &mut Formatter<'_>, row_idx: usize, widths: &Vec<usize>) -> std::fmt::Result {
        for col_idx in 0..(self.get_column_count() + 1) {
            let cell = self.get(row_idx, col_idx);
            let top_left = self.get_cell(row_idx as i64 - 1, col_idx as i64 - 1);
            let top_right = self.get_cell(row_idx as i64 - 1, col_idx as i64);
            let left = self.get_cell(row_idx as i64, col_idx as i64 - 1);

            let default_style = BorderStyle::default();
            let hor_style = self.get_horizontal_separator_style(row_idx).unwrap_or(&default_style);
            let vert_style = self.get_vertical_separator_style(col_idx).unwrap_or(&default_style);
            // cell corner symbol
            write!(f, "{}", get_common_cell_border_symbol(top_left, top_right, left, cell, hor_style.clone(), vert_style.clone()))?;

            // top border
            if col_idx == self.get_column_count() {
                continue;
            }
            for _ in 0..widths[col_idx] {
                write!(f, "{}", get_cell_border_symbols(self, row_idx, col_idx).0)?;
            }
        }
        Ok(())
    }

    /// Writes a single row to the formatter
    fn write_row(&self, f: &mut Formatter<'_>, row_idx: usize, widths: &Vec<usize>) -> std::fmt::Result {
        let height = self.get_row_height(row_idx);
        if height > 0 {
            for line in 0..height {
                for col_idx in 0..self.get_column_count() {
                    let cell = self.get(row_idx, col_idx).unwrap();
                    let symbols = get_cell_border_symbols(self, row_idx, col_idx);
                    if col_idx == 0 {
                        write!(f, "{}", symbols.1)?;
                    }

                    let content = cell.get_line(line).unwrap_or(String::new());
                    write!(f, "{content:width$}", width = widths[col_idx])?;
                    write!(f, "{}", symbols.2)?;
                }
                if line != height - 1 {
                    writeln!(f)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for FancyTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // capture empty tables
        if self.get_column_count() < 1 || self.get_row_count() < 1 {
            write!(f, "")?;
            return Ok(());
        }

        let widths = self.get_col_widths();
        for row_idx in 0..(self.get_row_count() + 1) {
            self.write_top_border(f, row_idx, &widths)?;

            if row_idx == self.get_row_count() {
                continue;
            }

            writeln!(f)?;
            self.write_row(f, row_idx, &widths)?;
        }

        Ok(())
    }
}
