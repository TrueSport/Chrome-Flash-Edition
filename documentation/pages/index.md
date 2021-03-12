# Overview

Amp is inspired by [Vim](https://vim.sourceforge.io)'s modal approach to
text editing, which is reflected in several of its default key bindings.
That similarity aside, there are several key differences.

Above all else, Amp aims to _keep things as simple as possible_. There are
already plenty of highly-configurable editors available. At its core, Amp aims
to minimize configuration and provide a great out-of-the-box experience. The
following sections describe some of the ideas central to the design of Amp that
differentiate it from other options.

### UX

Like Vim, Amp is a modal editor: keystrokes perform different functions based
on the current mode. Many familiar modes (insert, normal, select, etc.) are
available, as well as several new ones providing additional functionality.

### Essential Features

Amp's primary audience is _developers_.

Syntax highlighting, a fuzzy file finder, local symbol jump, and basic Git
integration are available without additional configuration or external
dependencies (e.g. plug-ins, ctags, external indexing binaries).

### Configuration

Amp shouldn't require any init