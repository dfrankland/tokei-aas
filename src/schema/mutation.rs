use crate::tokei::{tokei, Languages};
use super::state::State;
use juniper::FieldResult;
use std::sync::atomic;

pub struct Mutation;

#[juniper::object(Context=State)]
impl Mutation {
    fn add(context: &State, by: i32) -> i32 {
        context.0.fetch_add(by as isize, atomic::Ordering::Relaxed) as i32 + by
    }

    async fn tokei(repo: String, paths: Option<Vec<String>>, ignored: Option<Vec<String>>) -> FieldResult<Languages> {
        Ok(tokei(repo, paths, ignored).await?)
    }
}
