mod cli;
mod validators;

pub use cli::{run, Targets};
pub use validators::is_valid_target;
