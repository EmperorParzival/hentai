# hentai

The aptly named *hentai* crate provides an easy mechanism to consume nhentai's
public facing APIs in Rust.

As of now, hentai relies solely on these APIs instead of scraping their website.
However, this capability may be added in the near future.

hentai will provide information about a doujin given its six-digit code.
Alternatively, the JSON response from nhentai's
[/api/gallery/{id}](https://nhentai.net/api/gallery/165961) endpoint may be
provided directly.

hentai is based on the similar
[package for python](https://pypi.org/project/hentai/).

## Planned Features
[] Search doujins and other misc. features

[] CLI

[] Web scraping to get new & popular doujins
