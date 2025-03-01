mod automagic;
mod markers;
mod ordering;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::automagic::{InferFlow, InferFlowEach};
    pub use crate::markers::{IntoSystemRW, Reads, Writes};
    pub use crate::ordering::{read_before_write, write_before_read};
}
