<p align="center">
  <a href="https://crates.io/crates/hentai">
    <img alt="version" src="https://img.shields.io/crates/v/hentai.svg"></img>
  </a>
  <a href="https://docs.rs/hentai">
    <img alt="documentation" src="https://docs.rs/hentai/badge.svg"></img>
  </a>
  <a href="https://crates.io/crates/hentai">
    <img alt="downloads" src="https://img.shields.io/crates/d/hentai"></img>
  </a>
  <br />
  <a href="https://repl.it/github/EmperorParzival/hentai">
    <img alt="Run on Repl.it" src="https://repl.it/badge/github/EmperorParzival/hentai"></img>
  </a>
  <a href="https://gitpod.io/#https://github.com/EmperorParzival/hentai">
    <img alt="Gitpod ready-to-code" src="https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod"></img>
  </a>
  <br />
  <a href="https://github.com/EmperorParzival/hentai/blob/main/LICENSE">
    <img alt="license" src="https://img.shields.io/crates/l/hentai"></img>
  </a>
  <a href="https://github.com/EmperorParzival/hentai">
    <img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/EmperorParzival/hentai"></img>
  </a>
</p>

# hentai

The aptly named *hentai* crate provides an easy mechanism to consume nhentai's
public facing APIs in Rust. As of now, hentai relies solely on these APIs
instead of scraping their website. However, this capability may be added in the
near future.

hentai will provide information about a doujin given its six-digit code.
Alternatively, the JSON response from nhentai's
[/api/gallery/{id}](https://nhentai.net/api/gallery/165961) endpoint may be
provided directly.

hentai is based on the similar
[package for python](https://pypi.org/project/hentai/).

## Planned Features
- [ ] Search doujins and other misc. features
- [ ] CLI
- [ ] Web scraping to get new & popular doujins

## Install

### Using [cargo-edit](https://crates.io/crates/cargo-edit)
```shell
cargo add hentai
```

### Manually
Add the following to your `Cargo.toml`:
```toml
hentai = "0.2.1"
```

## Contribute

### Browser
[![Run on Repl.it](https://repl.it/badge/github/EmperorParzival/hentai)](https://repl.it/github/EmperorParzival/hentai)

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/EmperorParzival/hentai)

### Local
```shell
git clone https://github.com/EmperorParzival/hentai
cd hentai
cargo install --force --path .
```
