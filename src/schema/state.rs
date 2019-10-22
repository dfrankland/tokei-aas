use std::sync::{atomic, Arc};
use juniper::Context;

#[derive(Clone, Default)]
pub struct State(pub Arc<atomic::AtomicIsize>);

impl Context for State {}
