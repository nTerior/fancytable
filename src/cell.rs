use std::fmt::Alignment;
use std::str::FromStr;
use ansi_term::Style;
use unicode_width::UnicodeWidthStr;
use crate::style::border::{CellBorderStyle};
use crate::style::VerticalAlignment;

/// Splits the input into separate lines and returns them inside a [Vec]
fn multiline_from_string(s: String) -> Vec<String> {
    s.lines().map(String::from).collect()
}

/// A single, stylizable cell used inside [FancyTable](crate::FancyTable)
#[derive(Debug, PartialEq, Clone)]
pub struct FancyCell {
    content: Vec<String>,
    pub border_style: CellBorderStyle,
    pub padding: usize,
    pub horizontal_alignment: Alignment,
    pub vertical_alignment: VerticalAlignment,
    pub style: Style,
}

impl FancyCell {
    /// Creates a new [FancyCell] from an input string.
    /// The input supports multiline and unicode
    ///
    /// # Example
    /// ```
    /// use fancytable::FancyCell;
    /// let cell = FancyCell::new("Hello World!".to_string());
    /// let multiline = FancyCell::new("A 🆕\ncell".to_string());
    /// ```
    ///
    /// It is also create a cell using only a string
    /// ```
    /// use fancytable::FancyCell;
    /// let cell: FancyCell = "lorem ipsum".into();
    /// let cell: FancyCell = String::from("dolor sit").into();
    /// let cell: FancyCell = "amet".parse().unwrap();
    /// ```
    pub fn new(content: String) -> FancyCell {
        FancyCell {
            content: multiline_from_string(content),
            ..Self::default()
        }
    }

    /// Returns the multi line content of the cell.
    pub fn get_content(&self) -> &Vec<String> {
        &self.content
    }

    /// Returns the multi line content as a mutable [Vec]
    pub fn get_mut_content(&mut self) -> &mut Vec<String> {
        &mut self.content
    }

    /// Sets the content of the cell using a multiline string.
    pub fn set_content(&mut self, content: String) {
        self.content = multiline_from_string(content);
    }

    /// Returns a single padded line inside this cell.
    ///
    /// Returns [None] if the line does not exist.
    pub fn get_line(&self, line: usize) -> Option<String> {
        let line = self.content.get(line)?;
        let empty = "";
        let padded = format!("{empty:width$}{line}{empty:width$}", width=self.padding);
        Some(padded)
    }

    /// Returns a single, mutable line inside this cell.
    ///
    /// Returns [None] if the line does not exist.
    pub fn get_mut_line(&mut self, line: usize) -> Option<&mut String> {
        self.content.get_mut(line)
    }

    /// Sets a single line inside the cell.
    pub fn set_line(&mut self, line: usize, content: String) {
        self.content[line] = content;
    }

    /// Returns the height of the cell in lines.
    pub fn get_height(&self) -> usize {
        self.content.len()
    }

    /// Returns the unicode column width of this cell.
    /// See [UnicodeWidthStr::width] for more information.
    pub fn get_width(&self) -> usize {
        (0..self.content.len())
            .map(|i| strip_ansi_escapes::strip_str(self.get_line(i).unwrap()))
            .map(|s| s.width())
            .max()
            .unwrap_or(0)
    }
}

impl Default for FancyCell {
    fn default() -> Self {
        FancyCell {
            content: vec![" ".to_string()],
            border_style: Default::default(),
            padding: 1,
            horizontal_alignment: Alignment::Left,
            vertical_alignment: VerticalAlignment::default(),
            style: Style::default(),
        }
    }
}

impl Eq for FancyCell {}

impl From<String> for FancyCell {
    fn from(value: String) -> Self {
        FancyCell::new(value)
    }
}

impl From<&str> for FancyCell {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl FromStr for FancyCell {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}