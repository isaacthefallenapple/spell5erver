use ron;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{self, Debug, Display},
    iter::FromIterator,
    str::FromStr,
};

mod class;
mod component;
mod level;
mod range;
mod school;
mod time_unit;

pub use class::{Class, ClassSet};
pub use component::Components;
pub use level::Level;
pub use range::Range;
pub use school::{School, SchoolSet};
pub use time_unit::{CastingTime, Duration, TimeUnit};

/// `pluralize` returns the quantity followed by the singular name,
/// appending an 's' if `amt` != 1.
fn pluralize(amt: u32, s: &str) -> String {
    format!("{} {}{}", amt, s, if amt == 1 { "" } else { "s" })
}
