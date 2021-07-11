# swq-rs
Star Wars Quote and GIF Utility

## About swq-rs
Use your terminal often, and really want to grab that perfect quote or GIF for the situation? SWQ has you covered! It's easy to use, and quickly adds a GIF or full quote (with or without author) to your clipboard. swq-rs is a rewrite of the original Python+fire into Rust+clap for a faster, and more efficient codebase.

## Usage

- `swq quote KEYWORDS`

- `swq gif KEYWORDS`

That's it!

## Quick Install

- Cargo: `cargo install swq`

- AUR: `paru -S swq`

- Debian/Ubuntu & derivatives: Download `.deb` in [releases](https://github.com/TheCatster/swq-rs/releases)

A Tenor API key is recommended, to get one please go to [Tenor API](https://tenor.com/gifapi), and on first time use of the GIF command, it will prompt you to paste it in (the key is held in the keyring).

## To-Do:

- [x] Get quotes

- [x] Get GIFs (from Tenor)

- [x] Auto add to clipboard

- [ ] Add quotes

- [ ] TBD!

## Building:

Currently, it is only possible to build an amd64 binary. Dependencies include (find these for your specific distro):

- `libx11`

- `libxcb`

Simply run `cargo build --release` to get your release binary.

### Support:

The best way to support swq-rs is to contribute! Feel free to make PRs, open issues, and help us make this the best utility we can! If you're interested in adding a feature, simply open an issue.

### License:

Distributed under the GNU Public License (GPLv3) (See accompanying file LICENSE)

Keywords: rust, ureq, clap, cli
