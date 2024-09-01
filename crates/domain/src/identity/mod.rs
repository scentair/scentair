mod ordered;
mod unordered;

pub use ordered::*;
pub use unordered::*;

crate::new_ordered_identity!(UserId, "user_");
