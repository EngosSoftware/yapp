# Yet another preprocessor for mdBook

[![Crates.io][crates-badge]][crates-url]
[![Code coverage][cov-badge-mdbook-yapp]][cov-url]
![build Linux][build-badge-linux]
![build Windows][build-badge-windows]
![build macOs][build-badge-macos]
![build macOs arm64][build-badge-macos-arm64]
[![MIT licensed][mit-badge]][mit-license-url]
[![Apache 2.0 licensed][apache-badge]][apache-license-url]
[![Contributor Covenant][cc-badge]][cc-url]
[![Made by Human][made-by-human-badge]][made-by-human-url]

[crates-badge]: https://img.shields.io/crates/v/mdbook-yapp.svg
[crates-url]: https://crates.io/crates/mdbook-yapp
[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://opensource.org/licenses/MIT
[mit-license-url]: https://github.com/EngosSoftware/yapp/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: https://www.apache.org/licenses/LICENSE-2.0
[apache-license-url]: https://github.com/EngosSoftware/yapp/blob/main/LICENSE
[apache-notice-url]: https://github.com/EngosSoftware/yapp/blob/main/NOTICE
[build-badge-linux]: https://github.com/EngosSoftware/kivi/actions/workflows/build-linux.yml/badge.svg
[build-badge-windows]: https://github.com/EngosSoftware/kivi/actions/workflows/build-windows.yml/badge.svg
[build-badge-macos]: https://github.com/EngosSoftware/kivi/actions/workflows/build-macos.yml/badge.svg
[build-badge-macos-arm64]: https://github.com/EngosSoftware/kivi/actions/workflows/build-macos-arm64.yml/badge.svg
[cov-url]: https://crates.io/crates/coverio
[cov-badge-mdbook-yapp]: https://img.shields.io/badge/cov-23%25%20%E2%94%82%2033%25%20%E2%94%82%2021%25-f52020.svg
[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-blue.svg
[cc-url]: https://github.com/EngosSoftware/yapp/blob/main/CODE_OF_CONDUCT.md
[made-by-human-badge]: https://img.shields.io/badge/Made_by-HUMAN-red.svg
[made-by-human-url]: https://github.com/DariuszDepta
[repository-url]: https://github.com/EngosSoftware/yapp
[mdBook]: https://github.com/rust-lang/mdBook

**mdBook preprocessor for simple text replacements**

## Overview

[mdBook] preprocessor that simply replaces text in all chapters.

Phrases to be replaced with specified content are defined in plain-text configuration file.

## Installation

Install using Cargo:

```shell
$ cargo install mdbook-yapp
```

Configure this preprocessor by adding to your `book.toml` the following line:

```toml
[preprocessor.yapp]
```

Build you book as usual:

```shell
$ mdbook build
```

There should be a warning message displayed when no configuration file is found.

```shell
2023-11-11 12:01:02 [INFO] (mdbook::book): Book building has started
[WARNING][Yapp] configuration file not found, in current directory expected a file with the name starting with prefix: yapp
[WARNING][Yapp] configuration file not found, in current directory expected a file with the name starting with prefix: yapp
2023-11-11 12:01:03 [INFO] (mdbook::book): Running the html backend
```

Prepare the configuration file as described in the next section.

## Configuration

This preprocessor requires single configuration file in plain-text format.
This file's name should start with the prefix `yapp`. Letter case is not significant.
So names like `yapp`, `Yapp`, `Yappi`, `yapp.config` and similar will do.

Configuration file must contain pairs of lines of text.
First line is the phrase to be searched in the chapter and the second line is the replacement.
Having a configuration file named `yapp.config` with the following content:

```text
jd
John Doe
```

will inform this preprocessor to search for all `jd` instances in all chapters of the book
and to replace them with the text `John Doe`.

Configuration files may have empty lines, which are ignored.
Empty lines may make the configuration more readable when there are multiple replacements defined, like this:

```text
jd
John Doe

^note
**Note**:

@version
1.23.4
```

Replacements are done in the order defined in configuration file, so the substitutions may be chained, like this:

```text
a
ab

ab
abb

abb
abba

abba
ABBA
```

Every letter `a` in all chapters will be replaced with `ABBA`.

Note that each line in configuration file is trimmed before used. So configuration file with the content like this:

```text
a
    ab

         ab
 abb

   abb
                     abba

 abba
   ABBA
```

will have the same effect as the file in the previous example.

To preserve whitespaces in search pattern or replacement, enclose it in a single quotation mark:

```text
' a '
' b c '
```

or double quotation mark:

```text
" a "
" b c "
```

so the result of replacement for input `│ a │` will be `│ b c │`.

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions to [**yapp**][repository-url] are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.

---

Brought to you with 💙 by [Engos Software](https://engos.de)
