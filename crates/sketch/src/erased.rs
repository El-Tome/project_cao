use serde::{Deserialize, Serialize};

/// The ranks that no longer count.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct Erased {
    #[serde(default)]
    pub(crate) points: Vec<bool>,
    #[serde(default)]
    pub(crate) segments: Vec<bool>,
    #[serde(default)]
    pub(crate) circles: Vec<bool>,
}

impl Erased {
    pub(crate) fn holds(list: &[bool], rank: usize) -> bool {
        list.get(rank).copied().unwrap_or(false)
    }

    pub(crate) fn mark(list: &mut Vec<bool>, rank: usize) {
        if list.len() <= rank {
            list.resize(rank + 1, false);
        }
        list[rank] = true;
    }
}
