#[macro_use]
pub mod utils;

pub mod primitives;

pub use primitives::points;
pub use primitives::shapes;
pub use primitives::vertex;

pub mod bezier;
pub mod math;
