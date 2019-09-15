//! Various finishers for world generation, such as grass, snow, and trees.

mod clumped;
mod single;
mod snow;
mod tree;

pub use clumped::ClumpedFoliageFinisher;
pub use single::SingleFoliageFinisher;
pub use snow::SnowFinisher;
