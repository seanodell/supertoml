pub mod noop;
pub mod reference;
pub mod templating;

pub use noop::NoopPlugin;
pub use reference::{ReferencePlugin, ReferenceConfig};
pub use templating::TemplatingPlugin;
