# Security policy

## Reporting a vulnerability

Please **do not open a public issue** for a security problem. Use GitHub's
private reporting instead: on the repository, **Security → Report a
vulnerability**. The report stays between you and the maintainers until a fix
is out.

Say what you did, what you expected and what happened, and attach the file that
triggers it if there is one. A crash on a crafted file is worth reporting even
if you are not sure it is exploitable.

## What counts

CAO is a desktop program that opens files and runs nothing on a network. The
surface that matters is therefore what it reads:

- a `.caopart` file (a part and its history),
- a settings profile imported from a file,
- the list of recent files kept between sessions.

A file that makes the program run code of the file's choosing, read or write
outside what the user asked, or hang or exhaust memory without bound is a
vulnerability. A part that merely fails to open with an error message is a bug:
open a normal issue.

## Which versions

CAO has no release yet. Only the current `main` is supported; a fix lands there.

## What to expect

A report is acknowledged as soon as a maintainer reads it, and the reporter is
told when the fix is merged. The project is small and the people on it are few:
there is no guaranteed delay.
