pub mod border;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub enum VerticalAlignment {
    #[default]
    Top,
    Center,
    Bottom
}