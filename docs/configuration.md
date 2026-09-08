# Settings, profiles and customisation

See also: [navigation](navigation.md) · [viewport](viewport.md) ·
[interface](interface.md) · [architecture](ARCHITECTURE.md)

The aim of the project is that as much as possible be configurable rather than
hard-coded. It all lives in `cao_prefs`, with no dependency on the interface,
and it is all serialisable: a setting that cannot be written to a file can
neither be kept nor shared.

## The preferences screen

The ⚙ button of the toolbar, or `Cmd/Ctrl + ,`. Six sections:

| Section | What is set there |
| --- | --- |
| **Profils** | Switch, duplicate, delete, import, export, reset everything |
| **Viewport** | Orientation cube, grid and magnets, ruler, camera bounds |
| **Navigation** | Mouse habits, sensitivities, trackpad gestures |
| **Apparence** | The background, and every colour and width |
| **Raccourcis** | The key of each command |
| **Barre d'outils** | The place, the logo, and the arrangement of the buttons |

Everything is applied at once and written to disk in the same breath: there is
no "apply" button, so no setting lost because a window was closed.

## The profiles

A profile is a **complete set of settings**, with a name. It is the unit of
everything else: what is saved, what is reset, what is handed to somebody.

- The profile the interface calls "Par défaut" always exists and cannot be
  deleted: there is always something to come back to when an experiment goes
  wrong. It is stored under the key `default`, never under the sentence the
  user reads, so translating that sentence moves no identity — the comparison
  that protects it keeps matching.
- Two profiles cannot carry the same name — they would be impossible to tell
  apart in the list. A name already taken becomes "… 2".
- **Resetting everything** touches the active profile only.

### Sharing

A profile exports to a `.caoprofile` file (JSON), and imports from such a file
— somebody else's included. An imported profile arrives as one more profile, it
replaces nothing.

Every field has a default value, so **a profile written by a version that knew
fewer settings still loads**: the ones it does not know keep their original
value. Only a different version number is refused.

`settings.json` is at version 2. Version 1 named the profile that always
exists by a French sentence rather than by the `default` key, so a file
written by it is not read at all: the settings go back to their defaults. A
break rather than a migration, taken while there is nobody to migrate — the
rules for keeping older files readable will be written down when there is
somebody whose files have to survive.

Shortcuts use a "command" modifier worth Ctrl on Windows and Linux, Cmd on
macOS. A profile shared between machines therefore reads correctly on both
sides.

It is all written to `settings.json`, in the configuration folder of the OS. An
unreadable file does not stop the application from starting: it goes back to
the default values, failing which the user would have no way in to repair it.

## Appearance

### The background

Plain, a linear gradient at any angle, or a radial gradient whose centre and
radius one places. In both cases the colour is described by a **list of
stops** — they are added and removed at will. Two-colour gradients are what one
asks for first and three-colour ones just after; a list costs no more to draw.

The background is emitted in screen coordinates, with no camera: it does not
move when the view turns. Changing the kind of gradient keeps the colours
already chosen, otherwise trying all three would be tedious.

### The colours

Everything the viewport draws: the three axes and their width, the two levels
of grid, the three states of a sketch (free, constrained, another sketch), the
dimensions and the ones that only report, what a rule holds in place, the marks
of those rules, the tint of the areas, the matter, the hover, and the two
extrusion colours. No colour of the viewport is hard-coded any more: a colour
that lives in a constant somewhere is a colour nobody can change.

Colours are stored in sRGB — the space of colour pickers — and converted once,
where the geometry is built.

## The shortcuts

Click a shortcut then press the wanted key; the modifiers held at the moment of
the keystroke are taken with it. A key already in use is **taken away from the
other command**: two commands on the same key would make one of them
unreachable without saying why.

A shortcut never fires while a text field has the keyboard: typing "50" into a
dimension must not also trigger whatever 5 and 0 are bound to. And a shortcut
whose button is greyed out does nothing either.

## The toolbar

The place is free: at the top, at the bottom, on the left, on the right, or
floating.

The buttons are a **tree**. A group holds commands, separators and **other
groups, as deep as one likes**. The groups of the first level are the tabs; one
level down, a group is spread out where it is with its name beside it; deeper
still, it becomes a menu that opens on click — spreading out a third level
would push everything else off the bar, and there is no bottom to the possible
depth.

The editor allows selecting an entry, moving it up, moving it down, taking it
into the group just above, taking it back out, removing it, creating groups,
renaming them, and adding any command from the palette.

The groups of the standard bar are stored under keys too — `sketch`,
`drawing`, `circles`, `constraints`, `edit`, `extrusion` — and the interface
says how each one reads. A group the user made or renamed carries their own
words, and the interface hands them back untouched.

A bar arranged by hand **receives the tools added later**, each in the group
where the standard bar puts it. That is what makes the keys matter: a new
command finds its place by matching the group's name, so a translated name
would quietly have built a second group beside the first. Without any of it, a
bar reorganised once would never hear of a new tool again: the new buttons
would exist for a new profile and for nobody else. What the user arranged is
not touched; only what is missing is added.

A place for the **logo** already exists, with its text, until there is a
picture to put there.

## One single list of commands

A button and a shortcut are two ways of asking the same thing, so there is only
one list: `cao_prefs::Command`. Both paths end at the same function. Two
separate paths would end up diverging, and a shortcut that does *almost* what
its button does is worse than no shortcut.

That is also what makes adding a command a single gesture: it appears by itself
in the palette of the toolbar and in the list of shortcuts.

## What is not configurable yet

- The duration of the view-change animation (0.35 s) and the field of view of
  the camera (45°).
- The width of the cube borders that select an edge or a corner (22 %).
- The logo is a text: there is no image file to load yet.
- The colours of the interface itself (panels, buttons) are egui's; only the
  viewport follows the theme.
- Mouse gestures go through presets (Fusion, SolidWorks, Blender) and are not
  set button by button.
- No setting belongs to a part: everything is global.
