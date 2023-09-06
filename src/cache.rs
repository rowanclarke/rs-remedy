use std::{iter::Empty, path::PathBuf};

use crate::tokens::Tokens;

pub trait Cache: Send + Sync + 'static {
    type Item: Tokens;
    type Iter: Iterator<Item = Self::Item>;

    fn query_exact(rule: String, value: u64) -> Option<Self::Item>;
    fn query_all(rule: String) -> Self::Iter;
    fn invalidate(file: PathBuf, tokens: Self::Item);
}

impl Cache for () {
    type Item = ();
    type Iter = Empty<Self::Item>;

    fn query_exact(rule: String, value: u64) -> Option<Self::Item> {
        todo!()
    }

    fn query_all(rule: String) -> Self::Iter {
        todo!()
    }

    fn invalidate(file: PathBuf, tokens: Self::Item) {
        todo!()
    }
}
