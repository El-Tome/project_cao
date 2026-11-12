//! How a group of the tree is cut into what the bar draws: plain buttons, and
//! split buttons that pair a command with the menu of its variants.

use cao_prefs::{Command, GroupSize, Item};

use super::takes_a_menu;

/// The deepest level whose groups are spread out on the bar: a group below it
/// is a menu, and the commands it holds are its entries.
pub(in crate::screens::ribbon) const SPREAD_OUT: usize = 1;

/// One button out of a menu-depth group and the command before it, if any.
pub(in crate::screens::ribbon) struct Split<'a> {
    pub(in crate::screens::ribbon) before: Option<Command>,
    pub(in crate::screens::ribbon) group: &'a str,
    pub(in crate::screens::ribbon) items: &'a [Item],
}

impl<'a> Split<'a> {
    /// `item` as a split, when it is a group drawn as one at this depth.
    fn of(item: &'a Item, before: Option<Command>, depth: usize) -> Option<Self> {
        match item {
            Item::Group { name, size, items } if depth > SPREAD_OUT || *size == GroupSize::Menu => {
                Some(Self {
                    before,
                    group: name,
                    items,
                })
            }
            _ => None,
        }
    }

    /// Every command its menu reaches, the ones in its sub-menus included.
    pub(in crate::screens::ribbon) fn entries(&self) -> Vec<Command> {
        commands_within(self.items)
    }
}

fn commands_within(items: &[Item]) -> Vec<Command> {
    items
        .iter()
        .flat_map(|item| match item {
            Item::Command(command) => vec![*command],
            Item::Group { items, .. } => commands_within(items),
            Item::Separator => Vec::new(),
        })
        .collect()
}

/// An entry of the tree as the bar draws it: a command and the menu right
/// after it are one split button.
pub(in crate::screens::ribbon) enum Run<'a> {
    Plain(&'a Item),
    Split(Split<'a>),
}

pub(in crate::screens::ribbon) fn runs(items: &[Item], depth: usize) -> Vec<Run<'_>> {
    let mut runs = Vec::new();
    let mut rest = items.iter().peekable();
    while let Some(item) = rest.next() {
        if let Some(split) = Split::of(item, None, depth) {
            runs.push(Run::Split(split));
            continue;
        }
        let before = match item {
            Item::Command(command) if takes_a_menu(*command) => Some(*command),
            _ => None,
        };
        let paired = before.and_then(|_| Split::of(rest.peek()?, before, depth));
        match paired {
            Some(split) => {
                rest.next();
                runs.push(Run::Split(split));
            }
            None => runs.push(Run::Plain(item)),
        }
    }
    runs
}
