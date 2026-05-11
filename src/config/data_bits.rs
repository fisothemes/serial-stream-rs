#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum DataBits {
    #[default]
    Eight,
    Seven,
    Six,
    Five,
}
