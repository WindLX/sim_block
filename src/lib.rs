pub mod block;
pub mod model;
pub mod utils;

#[cfg(feature = "source")]
pub mod source;

#[cfg(feature = "sink")]
pub mod sink;

#[cfg(feature = "continuous")]
pub mod continuous;

#[cfg(feature = "vector")]
#[cfg(feature = "continuous")]
pub use continuous::vector::*;

#[cfg(feature = "discontinuous")]
pub mod discontinuous;
