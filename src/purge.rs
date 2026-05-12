#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Purge {
    Read,
    Write,
    Both,
}
