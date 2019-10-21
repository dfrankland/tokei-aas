use juniper::graphql_object;
use std::sync::{atomic, Arc};
use lazy_static::lazy_static;

#[derive(Clone, Default)]
pub struct State(Arc<atomic::AtomicIsize>);

impl juniper::Context for State {}

pub struct Query;

graphql_object!(Query: State |&self| {
    // GraphQL integers are signed and 32 bits long.
    field accumulator(&executor) -> i32 as "Current value of the accumulator" {
        executor.context().0.load(atomic::Ordering::Relaxed) as i32
    }
});

pub struct Mutation;

graphql_object!(Mutation: State |&self| {
    field add(&executor, by: i32) -> i32 as "Add given value to the accumulator." {
        executor.context().0.fetch_add(by as isize, atomic::Ordering::Relaxed) as i32 + by
    }
});

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

lazy_static! {
    pub static ref SCHEMA: Schema = Schema::new(Query, Mutation);
}
