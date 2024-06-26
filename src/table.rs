use crate::FancyCell;

/// A stylizable, rectangular table for pretty cli output.
#[derive(Debug, Eq, PartialEq, Default)]
pub struct FancyTable {
    /// Access: `cells[row][col]`
    cells: Vec<Vec<FancyCell>>,
    /// Set when adding a column to an empty table, so that a call on [FancyTable::add_rows] creates the correct result
    /// ONLY FOR INTERNAL USE!
    _added_column_first: bool,
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
        let max_columns = cells.iter()
            .map(|row| row.len())
            .max()
            .unwrap_or(0);

        // fills every row to the maximum number of columns
        // therefore, the table is now a rectangle
        for row in &mut cells {
            if row.len() < max_columns {
                for _ in 0..(max_columns - row.len()) {
                    row.push(FancyCell::default());
                }
            }
        }

        FancyTable {
            cells,
            _added_column_first: false,
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
        }
    }

    /// Adds a number of columns.
    /// The columns will be filled with default [FancyCells]s
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

        for row in &mut self.cells {
            for _ in 0..n {
                row.push(FancyCell::default());
            }
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
}
