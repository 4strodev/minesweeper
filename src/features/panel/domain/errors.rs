#[derive(Debug)]
/// This error should be launched when a given position is out of panel boundaries
pub struct OutOfBoundariesError {}

impl OutOfBoundariesError {
    pub fn new() -> OutOfBoundariesError {
        OutOfBoundariesError {}
    }
}

impl std::fmt::Display for OutOfBoundariesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position out of boundaries")
    }
}
