# Fork of Helix which adds support for Github-copilot

### To use:
### 1. Setup the copilot-lsp using my helper script:

1. Install [fish](https://fishshell.com/) (for my script), install [node.js](https://nodejs.org/en/download/) (which runs the copilot-lsp)

2. Run `fish copilot.fish --create-lsp` 
<br>This will download the copilot-lsp and put its in `/usr/local/bin/` (if you haven’t already)
3. Run `fish copilot.fish --auth` 
<br>This will create a github copilot-lsp access code in `~/.config/github-copilot/hosts.json` (if one doesn't already exist)

### 2. Choose keybindings to map to these copilot-commands, for example:
```
[keys.insert]
"C-w" = "copilot_apply_completion" 
"C-e" = "copilot_show_completion"

[keys.normal]
"C-e" = "copilot_toggle_auto_render"

[editor]
copilot-auto-render = false # defaults to true if omitted
```

### 3. Run helix with copilot: `hx -a`

With helix started with this flag ommited (eg `hx`), the copilot-lsp is not started at all and no completions are requested.

With helix started with `hx -a`, copilot-completions are requested from the copilot-lsp when entering insert mode, and anytime that the document changes while in insert mode (typing a character, deleting a word etc).

With `copilot-auto-render = true`, completions are rendered immediately after they are received from the copilot-lsp.

With `copilot-auto-render = false`, a completion will only render if `copilot_show_completion` is pressed after the completion was first requested. This behaviour then resets after the next completion is requested.

<div align="center">

<h1>
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="logo_dark.svg">
  <source media="(prefers-color-scheme: light)" srcset="logo_light.svg">
  <img alt="Helix" height="128" src="logo_light.svg">
</picture>
</h1>

[![Build status](https://github.com/helix-editor/helix/actions/workflows/build.yml/badge.svg)](https://github.com/helix-editor/helix/actions)
[![GitHub Release](https://img.shields.io/github/v/release/helix-editor/helix)](https://github.com/helix-editor/helix/releases/latest)
[![Documentation](https://shields.io/badge/-documentation-452859)](https://docs.helix-editor.com/)
[![GitHub contributors](https://img.shields.io/github/contributors/helix-editor/helix)](https://github.com/helix-editor/helix/graphs/contributors)
[![Matrix Space](https://img.shields.io/matrix/helix-community:matrix.org)](https://matrix.to/#/#helix-community:matrix.org)

</div>

![Screenshot](./screenshot.png)

A [Kakoune](https://github.com/mawww/kakoune) / [Neovim](https://github.com/neovim/neovim) inspired editor, written in Rust.

The editing model is very heavily based on Kakoune; during development I found
myself agreeing with most of Kakoune's design decisions.

For more information, see the [website](https://helix-editor.com) or
[documentation](https://docs.helix-editor.com/).

All shortcuts/keymaps can be found [in the documentation on the website](https://docs.helix-editor.com/keymap.html).

[Troubleshooting](https://github.com/helix-editor/helix/wiki/Troubleshooting)

# Features

- Vim-like modal editing
- Multiple selections
- Built-in language server support
- Smart, incremental syntax highlighting and code editing via tree-sitter

Although it's primarily a terminal-based editor, I am interested in exploring
a custom renderer (similar to Emacs) using wgpu or skulpin.

Note: Only certain languages have indentation definitions at the moment. Check
`runtime/queries/<lang>/` for `indents.scm`.

# Installation

[Installation documentation](https://docs.helix-editor.com/install.html).

[![Packaging status](https://repology.org/badge/vertical-allrepos/helix-editor.svg?exclude_unsupported=1)](https://repology.org/project/helix-editor/versions)

# Contributing

Contributing guidelines can be found [here](./docs/CONTRIBUTING.md).

# Getting help

Your question might already be answered on the [FAQ](https://github.com/helix-editor/helix/wiki/FAQ).

Discuss the project on the community [Matrix Space](https://matrix.to/#/#helix-community:matrix.org) (make sure to join `#helix-editor:matrix.org` if you're on a client that doesn't support Matrix Spaces yet).

# Credits

Thanks to [@jakenvac](https://github.com/jakenvac) for designing the logo!
