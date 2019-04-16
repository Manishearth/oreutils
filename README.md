## oreutils

[![Build Status](https://travis-ci.org/Manishearth/oreutils.svg?branch=master)](https://travis-ci.org/Manishearth/oreutils)
[![Current Version](https://meritbadge.herokuapp.com/oreutils)](https://crates.io/crates/oreutils)
[![License: MIT/Apache-2.0](https://img.shields.io/crates/l/oreutils.svg)](#license)


Currently a WIP

Oxidized coreutils, i.e. "coreutils without the C".

This project installs Rust CLI utilities that are reimaginations of various `coreutils` utilities. These are not drop-in replacements, however they typically cover most of the same functionality, and have often modernized various parts of the tool.

To get started:

```
cargo install oreutils
oreutils install
```

To upgrade your installed `oreutils`, run `oreutils upgrade`;

This tool currently installs:
 - `ripgrep`, a `grep` replacement
 - `exa`, an `ls` replacement
 - `fd`, a `find` replacement
 - `bat`, a `cat` replacement

More tools may be added. Please file an issue!

 [`ripgrep`]: http://github.com/burntsushi/ripgrep
 [`exa`]: https://the.exa.website/
 [`bat`]: https://github.com/sharkdp/bat
 [`fd`]: https://github.com/sharkdp/fd
