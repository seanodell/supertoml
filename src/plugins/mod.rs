pub mod noop;
pub mod reference;
pub mod templating;

pub use noop::NoopPlugin;
pub use reference::{ReferenceConfig, ReferencePlugin};
pub use templating::TemplatingPlugin;
