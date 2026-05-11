#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Port<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> Port<T> {
    pub fn new(port: T) -> Self {
        Self(port)
    }
}

impl<T: AsRef<str>> AsRef<str> for Port<T> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T: AsRef<str>> From<T> for Port<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
