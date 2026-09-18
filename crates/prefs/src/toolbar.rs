use serde::{Deserialize, Serialize};

use crate::command::Command;

mod standard;

/// Which edge of the window the toolbar is attached to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Edge {
    Top,
    Bottom,
    Left,
    Right,
    /// Not attached: a small window the user places wherever they like.
    Floating,
}

impl Edge {
    pub const ALL: [Self; 5] = [
        Self::Top,
        Self::Bottom,
        Self::Left,
        Self::Right,
        Self::Floating,
    ];

    /// Whether the toolbar runs down the window rather than across it.
    pub fn is_vertical(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }
}

/// One entry of the toolbar.
///
/// A group holds other entries, groups included, as deep as the user cares to
/// go. That is the whole point: the arrangement is theirs, not a shape baked
/// into the code.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Command(Command),
    Group { name: String, items: Vec<Item> },
    Separator,
}

impl Item {
    pub fn group(name: &str, items: Vec<Item>) -> Self {
        Self::Group {
            name: name.to_string(),
            items,
        }
    }

    fn children_mut(&mut self) -> Option<&mut Vec<Item>> {
        match self {
            Self::Group { items, .. } => Some(items),
            _ => None,
        }
    }
}

/// Where an entry sits in the tree: the rank of each group to walk through,
/// then the rank of the entry itself.
///
/// A path rather than a reference, because the settings screen moves entries
/// about while it holds one, and a reference into a tree being rearranged is
/// the shortest road to editing the wrong thing.
pub type Path = Vec<usize>;

/// The toolbar as the user has arranged it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolbarLayout {
    pub edge: Edge,
    /// Whether a logo is shown at the head of the bar.
    pub show_logo: bool,
    /// Text shown in place of a logo until there is a picture to show.
    pub logo_text: String,
    /// Show the label next to each button, or the label alone.
    pub show_labels: bool,
    /// Top-level entries. Groups at this level are the tabs of the ribbon.
    pub items: Vec<Item>,
}

impl ToolbarLayout {
    pub fn at(&self, path: &[usize]) -> Option<&Item> {
        let (&first, rest) = path.split_first()?;
        let mut item = self.items.get(first)?;
        for &step in rest {
            match item {
                Item::Group { items, .. } => item = items.get(step)?,
                _ => return None,
            }
        }
        Some(item)
    }

    fn siblings_mut(&mut self, path: &[usize]) -> Option<&mut Vec<Item>> {
        let (_, parents) = path.split_last()?;
        let mut items = &mut self.items;
        for &step in parents {
            items = items.get_mut(step)?.children_mut()?;
        }
        Some(items)
    }

    pub fn remove(&mut self, path: &[usize]) -> Option<Item> {
        let &last = path.last()?;
        let items = self.siblings_mut(path)?;
        (last < items.len()).then(|| items.remove(last))
    }

    /// Adds an entry as the last child of the group at `path`, or at the end of
    /// the toolbar when `path` is empty.
    pub fn push_into(&mut self, path: &[usize], item: Item) -> bool {
        let Some(items) = self.children_at_mut(path) else {
            return false;
        };
        items.push(item);
        true
    }

    fn children_at_mut(&mut self, path: &[usize]) -> Option<&mut Vec<Item>> {
        let mut items = &mut self.items;
        for &step in path {
            items = items.get_mut(step)?.children_mut()?;
        }
        Some(items)
    }

    /// Moves an entry one place earlier or later among its siblings, and returns where it ended up.
    pub fn shift(&mut self, path: &[usize], later: bool) -> Option<Path> {
        let &last = path.last()?;
        let items = self.siblings_mut(path)?;
        let target = if later {
            (last + 1 < items.len()).then_some(last + 1)?
        } else {
            last.checked_sub(1)?
        };
        items.swap(last, target);

        let mut moved = path.to_vec();
        *moved.last_mut()? = target;
        Some(moved)
    }

    /// Moves an entry into the group just before it, and returns where it
    /// ended up. Nothing happens when there is no group to go into.
    pub fn nest(&mut self, path: &[usize]) -> Option<Path> {
        let &last = path.last()?;
        let before = last.checked_sub(1)?;
        let items = self.siblings_mut(path)?;
        if !matches!(items.get(before)?, Item::Group { .. }) {
            return None;
        }

        let item = items.remove(last);
        let group = items.get_mut(before)?.children_mut()?;
        group.push(item);
        let rank = group.len() - 1;

        let mut moved = path.to_vec();
        *moved.last_mut()? = before;
        moved.push(rank);
        Some(moved)
    }

    /// Moves an entry out of its group, to just after it.
    pub fn unnest(&mut self, path: &[usize]) -> Option<Path> {
        if path.len() < 2 {
            return None;
        }
        let item = self.remove(path)?;

        let mut parent = path.to_vec();
        parent.pop();
        let &parent_rank = parent.last()?;
        let items = self.siblings_mut(&parent)?;
        items.insert(parent_rank + 1, item);

        let mut moved = parent;
        *moved.last_mut()? = parent_rank + 1;
        Some(moved)
    }

    pub fn rename(&mut self, path: &[usize], name: String) -> bool {
        let Some(&last) = path.last() else {
            return false;
        };
        let Some(items) = self.siblings_mut(path) else {
            return false;
        };
        match items.get_mut(last) {
            Some(Item::Group { name: existing, .. }) => {
                *existing = name;
                true
            }
            _ => false,
        }
    }
}

impl ToolbarLayout {
    /// Puts into this layout the commands the standard one has and it has not,
    /// each in the group it belongs to.
    ///
    /// A layout is saved with the profile, so a toolbar arranged once would
    /// never hear of a tool added later: the new buttons would exist for a
    /// fresh profile and for nobody else. What the user has arranged is left
    /// exactly as it is; only what is missing is added.
    pub fn adopt_new_commands(&mut self, reference: &Self) {
        let mut known = Vec::new();
        collect(&self.items, &mut known);

        let mut wanted = Vec::new();
        gather(&reference.items, &mut Vec::new(), &mut wanted);
        for (path, command) in wanted {
            if known.contains(&command) {
                continue;
            }
            let seat = seat_for(&mut self.items, &path);
            seat.push(Item::Command(command));
        }
    }
}

/// Every command a branch holds, however deep.
fn collect(items: &[Item], out: &mut Vec<Command>) {
    for item in items {
        match item {
            Item::Command(command) => out.push(*command),
            Item::Group { items, .. } => collect(items, out),
            Item::Separator => {}
        }
    }
}

/// Every command of a branch with the names of the groups it sits in.
fn gather(items: &[Item], path: &mut Vec<String>, out: &mut Vec<(Vec<String>, Command)>) {
    for item in items {
        match item {
            Item::Command(command) => out.push((path.clone(), *command)),
            Item::Group { name, items } => {
                path.push(name.clone());
                gather(items, path, out);
                path.pop();
            }
            Item::Separator => {}
        }
    }
}

/// The list a command should join, digging the groups of the path out of the
/// layout, or making them when they are not there.
fn seat_for<'a>(items: &'a mut Vec<Item>, path: &[String]) -> &'a mut Vec<Item> {
    let Some((name, rest)) = path.split_first() else {
        return items;
    };
    let found = items
        .iter()
        .position(|item| matches!(item, Item::Group { name: held, .. } if held == name));
    let index = match found {
        Some(index) => index,
        None => {
            items.push(Item::group(name, Vec::new()));
            items.len() - 1
        }
    };
    match &mut items[index] {
        Item::Group { items, .. } => seat_for(items, rest),
        _ => unreachable!("the entry was just found or made as a group"),
    }
}

#[cfg(test)]
mod tests;
