use super::state::State;
use std::sync::atomic;

pub struct Query;

#[juniper::object(Context=State)]
impl Query {
    fn accumulator(context: &State) -> i32 {
        context.0.load(atomic::Ordering::Relaxed) as i32
    }
}
