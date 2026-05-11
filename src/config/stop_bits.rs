#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum StopBits {
    #[default]
    One,
    OnePointFive,
    Two,
}
