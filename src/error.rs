#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Msg(String),
    NulError,
}

impl Error {
    pub fn make_msg(msg: &str) -> Error {
        Error::Msg(msg.to_string())
    }
}
