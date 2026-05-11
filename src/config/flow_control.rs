#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FlowControl {
    #[default]
    None,
    Hardware,
    Software,
}
