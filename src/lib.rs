mod automagic;
mod markers;
mod ordering;

#[cfg(test)]
mod tests;

pub use automagic::InferFlow;
pub use markers::{IntoSystemRW, Reads, Writes};
pub use ordering::{read_before_write, write_before_read};
