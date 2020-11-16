use std::fmt::{self, Display, Formatter};

use super::{Ret, RetOne};

pub enum RetTypes {
    Usize(RetOne<usize>),
    Isize(RetOne<isize>),
    String(RetOne<String>),
    UsizeString(Ret<usize, String>),
    VecIsizeIsize(Ret<Vec<isize>, isize>),
}

impl Display for RetTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // does someone know how to reduce this construction?
        match self {
            RetTypes::Usize(e) => write!(f, "{}", e),
            RetTypes::Isize(e) => write!(f, "{}", e),
            RetTypes::String(e) => write!(f, "{}", e),
            RetTypes::UsizeString(e) => write!(f, "{}", e),
            RetTypes::VecIsizeIsize(e) => write!(f, "{}", e),
        }
    }
}
