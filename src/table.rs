use crate::FancyCell;
use crate::style::border::{BorderStyle, TableOutline};

/// A stylizable, rectangular table for pretty cli output.
#[derive(Debug, Eq, PartialEq, Default)]
pub struct FancyTable {
    /// Access: `cells[row][col]`
    cells: Vec<Vec<FancyCell>>,
    /// Set when adding a column to an empty table, so that a call on [FancyTable::add_rows] creates the correct result
    /// ONLY FOR INTERNAL USE!
    _added_column_first: bool,
    vertical_separator_styles: Vec<BorderStyle>,
    horizontal_separator_styles: Vec<BorderStyle>,
    pub outline: TableOutline,
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

        let mut vertical_separators: usize = 0;
        if columns > 1 {
            vertical_separators = columns - 1;
        }

        let mut horizontal_separators: usize = 0;
        if cells.len() > 1 {
            horizontal_separators = cells.len() - 1;
        }

        FancyTable {
            vertical_separator_styles: vec![BorderStyle::default(); vertical_separators],
            horizontal_separator_styles: vec![BorderStyle::default(); horizontal_separators],
            _added_column_first: false,
            outline: TableOutline::default(),
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

    /// Returns a reference to the [FancyCell] at the position (row_idx, col_idx) in the table
    /// Returns None if not found
    pub fn get(&self, row_idx: usize, col_idx: usize) -> Option<&FancyCell> {
        let row = self.cells.get(row_idx)?;
        row.get(col_idx)
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

    /// Returns the style for a single vertical separator.
    pub fn get_vertical_separator_style(&self, idx: usize) -> Option<&BorderStyle> {
        self.vertical_separator_styles.get(idx)
    }

    /// Returns the style for a single horizontal separator.
    pub fn get_horizontal_separator_style(&self, idx: usize) -> Option<&BorderStyle> {
        self.horizontal_separator_styles.get(idx)
    }
}
