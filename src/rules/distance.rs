#[derive(Debug, Clone, Copy)]
pub struct Distance(Option<usize>);

impl Distance {
    pub const fn finite(value: usize) -> Self {
        Self(Some(value))
    }

    pub const fn infinite() -> Self {
        Self(None)
    }

    pub const fn is_infinite(&self) -> bool {
        self.0.is_none()
    }

    pub const fn value(&self) -> Option<usize> {
        self.0
    }
}
