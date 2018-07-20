#[derive(Debug, Fail)]
pub enum ParseError {
    #[fail(display = "Algorithm parsing error")]
    Algorithm,
}