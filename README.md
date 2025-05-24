# maex

[![GitHub License](https://img.shields.io/github/license/PRO-2684/maex?logo=opensourceinitiative)](https://github.com/PRO-2684/maex/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/maex/release.yml?logo=githubactions)](https://github.com/PRO-2684/maex/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/maex?logo=githubactions)](https://github.com/PRO-2684/maex/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/maex/total?logo=github)](https://github.com/PRO-2684/maex/releases)
[![Crates.io Version](https://img.shields.io/crates/v/maex?logo=rust)](https://crates.io/crates/maex)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/maex?logo=rust)](https://crates.io/crates/maex)
[![docs.rs](https://img.shields.io/docsrs/maex?logo=rust)](https://docs.rs/maex)

Minecraft (Java Edition) Assets Extractor.

## 📥 Installation

### Using [`binstall`](https://github.com/cargo-bins/cargo-binstall)

```shell
cargo binstall maex
```

### Downloading from Releases

Navigate to the [Releases page](https://github.com/PRO-2684/maex/releases) and download respective binary for your platform. Make sure to give it execute permissions.

### Compiling from Source

```shell
cargo install maex
```

## 💡 Examples

TODO

## 📖 Usage

```shell
$ maex
Usage: maex <index> [<output>]
```

Where `<index>` is the path to the index JSON file, and `<output>` is the output directory where the assets will be extracted to. If `<output>` is not specified, the assets will be extracted to a directory next to the executable, with timestamp as the name.

Refer to the [documentation](https://docs.rs/maex) for using this crate as a library.

## 📂 Assets Folder

The `assets` folder, usually located directly under `.minecraft`, consists of 3 folders:

- `index`: This folder contains several JSON files for indexing the assets, each for a specific game version. Each JSON file contains a single key `objects` at the top level, which is a map of asset paths to their corresponding hash and size.
- `objects`: This folder contains the actual assets, which are stored in a hashed format. Specifically, denote the hash as `hash`, the asset will be stored in `objects/hash[0..2]/hash`.
- `skins`: Structured like `objects`, containing skin images, but I don't know where their indices are stored.

## 🎉 Credits

TODO
