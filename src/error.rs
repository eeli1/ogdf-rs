#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Msg(String),
    NulError,
}

pub fn make_msg(msg: &str) -> Error {
    Error::Msg(msg.to_string())
}
