mod automagic;
mod markers;
mod ordering;

#[cfg(test)]
mod tests;

pub use automagic::{InferFlow, InferFlowEach};
pub use markers::{IntoSystemRW, Reads, Writes};
pub use ordering::{read_before_write, write_before_read};
