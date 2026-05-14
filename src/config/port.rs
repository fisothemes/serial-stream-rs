use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Port<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> Port<T> {
    pub fn new(port: T) -> Self {
        Self(port)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> T {
        self.0
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

impl<'a> Port<&'a str> {
    pub fn to_owned(&self) -> Port<String> {
        Port(self.0.to_string())
    }
}

impl<T: AsRef<str>> fmt::Display for Port<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_ref())
    }
}
