use juniper::Context;
use std::sync::{atomic, Arc};

#[derive(Clone, Default)]
pub struct State(pub Arc<atomic::AtomicIsize>);

impl Context for State {}
