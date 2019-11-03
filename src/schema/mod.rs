mod mutation;
mod query;
mod state;

pub use self::mutation::Mutation;
pub use self::query::Query;
pub use self::state::State;
use lazy_static::lazy_static;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

lazy_static! {
    pub static ref SCHEMA: Schema = Schema::new(Query, Mutation);
}
