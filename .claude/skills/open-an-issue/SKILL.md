---
name: open-an-issue
description: Ask and argue before writing an issue on this CAO repository. Use whenever an issue is about to be opened — a feature asked for in a sentence, a decision to record, a bug whose right behaviour is not obvious — and before writing a single line of its body.
---

# Opening an issue

An issue is where the reasoning goes. It is read by whoever picks the work up,
weeks later, and it is the only thing standing between "what was asked" and
"what was built".

**The failure mode is not badly written issues. It is issues written from the
first sentence somebody said.** That sentence is the first idea, not the need.
Written down as-is, it comes back as work that answers the sentence.

Two from this repository, both checkable:

- **#180** said "part.bin caches the whole rebuilt part". Building it had to
  decide two things the issue had not: whether `.bin` meant a binary format,
  and **when** the cache is written. The first answer to the second — at every
  save — was wrong, and was undone once the human said "only when the part is
  closed". Nobody had asked.
- **#178** shipped with three open questions in its body, left to whoever
  implemented it. Two were decisions, not details.

## Before writing a line of it

Ask, and do not write until the answers are in. Writing the issue and asking
afterwards is the same as not asking: the shape is already fixed, and the human
reads their own words back instead of thinking.

1. **What is the software able to do afterwards, seen by the user?** Not the
   mechanism — the gesture, the screen, what appears. If that cannot be said in
   a sentence, the issue is not one issue.
2. **What must not break?** The answer names the tests to keep, and often a
   whole part of the work nobody had counted.
3. **Argue at least one point.** Propose another way of doing it, with what it
   costs, so the human chooses against something rather than by default. The
   aim is not to be right; it is that the decision exists. An issue that only
   repeats the request hides the choice that was never made.
4. **Walk the edge cases out loud, one at a time**, and get an answer for each.
   "And if the face no longer exists?" "And if two of them fit?" "And if the
   file was written by an older version?" One question, one answer, then the
   next: a list of five gets one answer for the easiest of them.
5. **Ask what is out of scope**, and write it down. An issue with no borders is
   an issue that grows in the branch.

## What the issue says

- **Title**: a sentence saying what the software can do then — the same voice
  as a commit, `An arc keeps its centre when the trait it leans on moves`, not
  `Fix arc bug`.
- **Why now**: what is annoying today, in the user's terms.
- **What it does**, seen from the outside.
- **What must not break.**
- **Out of scope**, explicitly.
- **What was decided on the way**, and what was set aside, with the reason.
  That is the part nobody can reconstruct later.

English, as everything a developer reads. A sentence the user sees is quoted in
French inside it — the language test skips what sits between quotes.

## An issue that decides rather than builds

Label it `decision`. It carries the options, **what each one costs**, and a
recommendation. Never a neutral catalogue: whoever reads it a month later needs
to know what the person closest to the code thought.

It closes on a decision written down, not on code — and the issues that build
on it come after, naming it.

## When not to interrogate

A typo, a crash with one obvious right behaviour, a rule already written in
`CLAUDE.md` that the code breaks: open the issue and get on with it. The
questions above are for an issue about **behaviour**, an **architecture
decision**, or anything with more than one sensible shape.

The test is simple: if two competent people could build two different things
from the sentence, ask.

## One issue, one behaviour

If the answers name three things, that is three issues. Open them, and say in
each what it sits on. An epic carries none of the `todo` / `in-progress` /
`backlog` labels — it is tracked by its sub-issues.

Then hand the work over the way `open-a-task` says: a branch from the issue, and
nothing on `main` any other way.
