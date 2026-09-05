use serde::{Deserialize, Serialize};

use crate::command::Command;

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

    pub fn label(self) -> &'static str {
        match self {
            Self::Top => "En haut",
            Self::Bottom => "En bas",
            Self::Left => "À gauche",
            Self::Right => "À droite",
            Self::Floating => "Flottante",
        }
    }

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

    pub fn label(&self) -> String {
        match self {
            Self::Command(command) => command.label().to_string(),
            Self::Group { name, .. } => name.clone(),
            Self::Separator => "— séparateur —".to_string(),
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

    /// Moves an entry one place earlier or later among its siblings, and
    /// returns where it ended up.
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

impl Default for ToolbarLayout {
    fn default() -> Self {
        use Command as C;
        Self {
            edge: Edge::Top,
            show_logo: false,
            logo_text: "CAO".to_string(),
            show_labels: true,
            items: vec![
                Item::group(
                    "Esquisse",
                    vec![
                        Item::Command(C::NewSketch),
                        Item::Separator,
                        Item::group(
                            "Dessin",
                            vec![
                                Item::Command(C::ToolSelect),
                                Item::Command(C::ToolLine),
                                Item::Command(C::ToolRectangle),
                                Item::Command(C::ToolCircle),
                                Item::Command(C::ToolPoint),
                                Item::Command(C::ToolDimension),
                            ],
                        ),
                        Item::Command(C::RecenterOnSketch),
                        Item::Command(C::FinishSketch),
                        Item::Separator,
                        Item::group("Édition", vec![Item::Command(C::Undo), Item::Command(C::Redo)]),
                    ],
                ),
                Item::group(
                    "Extrusion",
                    vec![
                        Item::Command(C::ExtrusionAdd),
                        Item::Command(C::ExtrusionCut),
                        Item::Separator,
                        Item::Command(C::ExtrusionStraight),
                        Item::Command(C::ExtrusionRevolution),
                    ],
                ),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layout() -> ToolbarLayout {
        ToolbarLayout {
            items: vec![
                Item::Command(Command::Undo),
                Item::group("G", vec![Item::Command(Command::Redo)]),
                Item::Command(Command::NewSketch),
            ],
            ..ToolbarLayout::default()
        }
    }

    #[test]
    fn an_entry_is_found_by_its_path() {
        let layout = layout();
        assert_eq!(layout.at(&[0]), Some(&Item::Command(Command::Undo)));
        assert_eq!(layout.at(&[1, 0]), Some(&Item::Command(Command::Redo)));
        assert_eq!(layout.at(&[9]), None);
        assert_eq!(layout.at(&[0, 0]), None, "une commande n'a pas d'enfants");
    }

    #[test]
    fn shifting_swaps_with_the_neighbour() {
        let mut layout = layout();
        assert_eq!(layout.shift(&[0], true), Some(vec![1]));
        assert_eq!(layout.at(&[1]), Some(&Item::Command(Command::Undo)));
        assert_eq!(layout.shift(&[0], false), None, "rien avant le premier");
    }

    /// Groups within groups within groups : nesting has no floor.
    #[test]
    fn nesting_goes_as_deep_as_asked() {
        let mut layout = ToolbarLayout {
            items: vec![
                Item::group("A", vec![]),
                Item::group("B", vec![]),
                Item::Command(Command::Undo),
            ],
            ..ToolbarLayout::default()
        };

        assert_eq!(layout.nest(&[2]), Some(vec![1, 0]), "dans B");
        assert_eq!(layout.nest(&[1]), Some(vec![0, 0]), "B dans A");
        assert_eq!(layout.at(&[0, 0, 0]), Some(&Item::Command(Command::Undo)));
    }

    #[test]
    fn an_entry_comes_back_out_next_to_its_group() {
        let mut layout = layout();
        assert_eq!(layout.unnest(&[1, 0]), Some(vec![2]));
        assert_eq!(layout.at(&[2]), Some(&Item::Command(Command::Redo)));
        assert_eq!(layout.at(&[3]), Some(&Item::Command(Command::NewSketch)));
        assert_eq!(layout.unnest(&[0]), None, "déjà à la racine");
    }

    #[test]
    fn nesting_needs_a_group_to_go_into() {
        let mut layout = layout();
        assert_eq!(layout.nest(&[1]), None, "avant lui est une commande");
    }

    #[test]
    fn an_entry_can_be_taken_out_and_put_back() {
        let mut layout = layout();
        let taken = layout.remove(&[1, 0]).expect("la commande du groupe");
        assert_eq!(taken, Item::Command(Command::Redo));
        assert!(layout.push_into(&[1], taken));
        assert_eq!(layout.at(&[1, 0]), Some(&Item::Command(Command::Redo)));
    }

    #[test]
    fn only_a_group_can_be_renamed() {
        let mut layout = layout();
        assert!(layout.rename(&[1], "Autre".to_string()));
        assert_eq!(layout.at(&[1]).map(Item::label), Some("Autre".to_string()));
        assert!(!layout.rename(&[0], "Rien".to_string()));
    }
}
