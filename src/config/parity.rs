#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Parity {
    #[default]
    None,
    Even,
    Odd,
    Mark,
    Space,
}
