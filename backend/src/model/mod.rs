use sea_orm::entity::prelude::*;
use sea_orm::Iterable;

mod users;
mod orders;

pub use users::*;
pub use orders::*;