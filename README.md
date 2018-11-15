# voca_rs [![Build Status](https://travis-ci.org/e1r0nd/voca_rs.svg?branch=master)](https://travis-ci.org/e1r0nd/voca_rs) [![Crates version](http://meritbadge.herokuapp.com/voca_rs)](https://crates.io/crates/voca_rs) [![codecov](https://codecov.io/gh/e1r0nd/voca_rs/branch/master/graph/badge.svg)](https://codecov.io/gh/e1r0nd/voca_rs) [![license](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

**Voca_rs is a Rust library for manipulating strings.**

Voca_rs is inspired by [Voca.js](https://vocajs.com/) and [string.py](https://docs.python.org/3.4/library/string.html)

## TL;DR

```rust
use voca_rs::*;

let input_string = "LazyLoad with XMLHttpRequest and snake_case";
let string_in_words = split::words(&input_string);
// => ["Lazy", "Load", "with", "XML", "Http", "Request", "and", "snake", "case"]
let words_in_string = &string_in_words.join(" ");
// => "Lazy Load with XML Http Request and snake case"
let snake_string = case::snake_case(chop::slice(&words_in_string, 13, 28));
// => "xml_http_request"
let truncated_string = chop::prune(&words_in_string, 15, "");
// => "Lazy Load..."
```

## Documentation

See the complete documentation at https://e1r0nd.github.io/voca_rs/

Run tests: `cargo test`<br>
Build docs: `cargo doc` -> `./target/doc/voca_rs/index.html` <br>
Build a project: `cargo build` -> `./target/debug`

## Functions

#### Case

- [camel_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.camel_case.html)
- [capitalize](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.capitalize.html)
- [decapitalize](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.decapitalize.html)
- [kebab_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.kebab_case.html)
- [lower_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.lower_case.html)
- [pascal_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.pascal_case.html)
- [shouty_kebab_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.shouty_kebab_case.html)
- [shouty_snake_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.shouty_snake_case.html)
- [snake_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.snake_case.html)
- [swap_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.swap_case.html)
- [title_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.title_case.html)
- [upper_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.upper_case.html)

#### Chop

- [char_at](https://e1r0nd.github.io/voca_rs/voca_rs/chop/fn.char_at.html)
- [first](https://e1r0nd.github.io/voca_rs/voca_rs/chop/fn.first.html)
- [grapheme_at](https://e1r0nd.github.io/voca_rs/voca_rs/chop/fn.grapheme_at.html)
- [last](https://e1r0nd.github.io/voca_rs/voca_rs/chop/fn.last.html)
- [prune](https://e1r0nd.github.io/voca_rs/voca_rs/chop/fn.prune.html)
- [slice](https://e1r0nd.github.io/voca_rs/voca_rs/chop/fn.slice.html)
- [substr](https://e1r0nd.github.io/voca_rs/voca_rs/chop/fn.substr.html)
- [substring](https://e1r0nd.github.io/voca_rs/voca_rs/chop/fn.substring.html)
- [truncate](https://e1r0nd.github.io/voca_rs/voca_rs/chop/fn.truncate.html)

#### Count

- [count](https://e1r0nd.github.io/voca_rs/voca_rs/count/fn.count.html)
- [count_graphemes](https://e1r0nd.github.io/voca_rs/voca_rs/count/fn.count_graphemes.html)
- [count_substrings](https://e1r0nd.github.io/voca_rs/voca_rs/count/fn.count_substrings.html)
- [count_words](https://e1r0nd.github.io/voca_rs/voca_rs/count/fn.count_words.html)

#### Escape

- [escape_html](https://e1r0nd.github.io/voca_rs/voca_rs/escape/fn.escape_html.html)
- [escape_regexp](https://e1r0nd.github.io/voca_rs/voca_rs/escape/fn.escape_regexp.html)
- [unescape_html](https://e1r0nd.github.io/voca_rs/voca_rs/escape/fn.unescape_html.html)

#### Index

- [index_of](https://e1r0nd.github.io/voca_rs/voca_rs/index/fn.index_of.html)
- [last_index_of](https://e1r0nd.github.io/voca_rs/voca_rs/index/fn.last_index_of.html)

#### Manipulate

- [insert](https://e1r0nd.github.io/voca_rs/voca_rs/manipulate/fn.insert.html)
- [latinise](https://e1r0nd.github.io/voca_rs/voca_rs/manipulate/fn.latinise.html)
- [pad](https://e1r0nd.github.io/voca_rs/voca_rs/manipulate/fn.pad.html)
- [pad_left](https://e1r0nd.github.io/voca_rs/voca_rs/manipulate/fn.pad_left.html)
- [pad_right](https://e1r0nd.github.io/voca_rs/voca_rs/manipulate/fn.pad_right.html)
- [repeat](https://e1r0nd.github.io/voca_rs/voca_rs/manipulate/fn.repeat.html)
- [replace](https://e1r0nd.github.io/voca_rs/voca_rs/manipulate/fn.replace.html)
- [replace_all](https://e1r0nd.github.io/voca_rs/voca_rs/manipulate/fn.replace_all.html)
- [reverse](https://e1r0nd.github.io/voca_rs/voca_rs/manipulate/fn.reverse.html)
- [reverse_grapheme](https://e1r0nd.github.io/voca_rs/voca_rs/manipulate/fn.reverse_grapheme.html)
- [slugify](https://e1r0nd.github.io/voca_rs/voca_rs/manipulate/fn.slugify.html)
- [splice](https://e1r0nd.github.io/voca_rs/voca_rs/manipulate/fn.splice.html)
- [trim](https://e1r0nd.github.io/voca_rs/voca_rs/manipulate/fn.trim.html)
- [trim_left](https://e1r0nd.github.io/voca_rs/voca_rs/manipulate/fn.trim_left.html)
- [trim_right](https://e1r0nd.github.io/voca_rs/voca_rs/manipulate/fn.trim_right.html)

#### Query

- [ends_with](https://e1r0nd.github.io/voca_rs/voca_rs/query/fn.ends_with.html)
- [includes](https://e1r0nd.github.io/voca_rs/voca_rs/query/fn.includes.html)
- [is_alpha](https://e1r0nd.github.io/voca_rs/voca_rs/query/fn.is_alpha.html)
- [is_alphadigit](https://e1r0nd.github.io/voca_rs/voca_rs/query/fn.is_alphadigit.html)
- [is_blank](https://e1r0nd.github.io/voca_rs/voca_rs/query/fn.is_blank.html)
- [is_digit](https://e1r0nd.github.io/voca_rs/voca_rs/query/fn.is_digit.html)
- [is_empty](https://e1r0nd.github.io/voca_rs/voca_rs/query/fn.is_empty.html)
- [is_lowercase](https://e1r0nd.github.io/voca_rs/voca_rs/query/fn.is_lowercase.html)
- [is_numeric](https://e1r0nd.github.io/voca_rs/voca_rs/query/fn.is_numeric.html)
- [is_uppercase](https://e1r0nd.github.io/voca_rs/voca_rs/query/fn.is_uppercase.html)
- [query](https://e1r0nd.github.io/voca_rs/voca_rs/query/fn.query.html)
- [starts_with](https://e1r0nd.github.io/voca_rs/voca_rs/query/fn.starts_with.html)

#### Split

- [chars](https://e1r0nd.github.io/voca_rs/voca_rs/split/fn.chars.html)
- [graphemes](https://e1r0nd.github.io/voca_rs/voca_rs/split/fn.graphemes.html)
- [split](https://e1r0nd.github.io/voca_rs/voca_rs/split/fn.split.html)
- [words](https://e1r0nd.github.io/voca_rs/voca_rs/split/fn.words.html)

#### Strip

- [strip_bom](https://e1r0nd.github.io/voca_rs/voca_rs/split/fn.strip_bom.html)

#### Utils

- [ASCII_LETTERS](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.ASCII_LETTERS.html)
- [ASCII_LOWERCASE](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.ASCII_LOWERCASE.html)
- [ASCII_UPPERCASE](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.ASCII_UPPERCASE.html)
- [DIGITS](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.DIGITS.html)
- [HEXDIGITS](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.HEXDIGITS.html)
- [OCTDIGITS](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.OCTDIGITS.html)
- [PUNCTUATION](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.PUNCTUATION.html)
- [PRINTABLE](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.PRINTABLE.html)
- [WHITESPACE](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.WHITESPACE.html)
- [VERSION](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.VERSION.html)

## Roadmap

#### Chop

- codePointAt - [link](https://vocajs.com/#codePointAt)
- max - [link](https://www.tutorialspoint.com/python3/string_max.htm)
- min - [link](https://www.tutorialspoint.com/python3/string_min.htm)

#### Count

- countWhere - [link](https://vocajs.com/#countWhere)

#### Format

- format - [link](https://docs.python.org/3.4/library/string.html#string.Formatter.format)
- sprintf - [link](https://vocajs.com/#sprintf)
- vprintf - [link](https://vocajs.com/#vprintf)

#### Index

- search - [link](https://vocajs.com/#search)

#### Manipulate

- expand_tabs - [link](https://www.tutorialspoint.com/python3/string_expandtabs.htm)
- tr - [link](https://vocajs.com/#tr)
- wordWrap - [link](https://vocajs.com/#wordWrap)
- zfill - [link](https://www.tutorialspoint.com/python3/string_zfill.htm)

#### Query

- is_title - [link](https://www.tutorialspoint.com/python3/string_istitle.htm)
- matches - [link](https://vocajs.com/#matches)

#### Split

- codePoints - [link](https://vocajs.com/#codePoints)

#### Strip

- stripTags - [link](https://vocajs.com/#stripTags)

## Copyright

coded by Anatol Marezhanyi [@e1r0nd_crg](https://twitter.com/e1r0nd_crg)

https://linkedin.com/in/merezhany/<br>
https://www.youtube.com/c/AnatolMerezhanyi

## License

Licensed under [MIT License](LICENSE)
