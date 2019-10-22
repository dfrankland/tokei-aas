mod state;
mod query;
mod mutation;

use lazy_static::lazy_static;
pub use self::state::State;
pub use self::query::Query;
pub use self::mutation::Mutation;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

lazy_static! {
    pub static ref SCHEMA: Schema = Schema::new(Query, Mutation);
}
