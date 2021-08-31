# voca_rs
[![Crates version](https://img.shields.io/crates/v/voca_rs.svg)](https://crates.io/crates/voca_rs)
[![dependency status](https://deps.rs/crate/voca_rs/1.13.1/status.svg)](https://deps.rs/crate/voca_rs/1.13.1)
[![Build Status](https://app.travis-ci.com/a-merezhanyi/voca_rs.svg?branch=master)](https://app.travis-ci.com/a-merezhanyi/voca_rs)
[![codecov](https://codecov.io/gh/a-merezhanyi/voca_rs/branch/master/graph/badge.svg?token=uSEi0L8ivo)](https://codecov.io/gh/a-merezhanyi/voca_rs)
[![license](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

**Voca_rs is a Rust library for manipulating [unicode] strings.**

Voca_rs is implemented on Foreign Types, i.e. `String` and `str`. Respects Unicode.

Voca_rs is inspired by [Voca.js](https://vocajs.com/) (JavaScript), [string.py](https://docs.python.org/3.4/library/string.html) (Python), [Inflector](https://github.com/whatisinternet/inflector) (Rust), and [Grafite](https://docs.grafite.ca/utilities/helpers/) (PHP).

## TL;DR

Using functions:

```rust
use voca_rs::*;

let input_string = "LazyLoad with XMLHttpRequest and snake_case";
let string_in_words = split::words(&input_string);
// => ["Lazy", "Load", "with", "XML", "Http", "Request", "and", "snake", "case"]
let words_in_string = &string_in_words.join(" ");
// => "Lazy Load with XML Http Request and snake case"
let truncated_string = chop::prune(&words_in_string, 21, "");
// => "Lazy Load with XML..."
let sliced_string = chop::slice(&truncated_string, 5, -2);
// => "Load with XML."
let snaked_string = case::snake_case(&sliced_string);
// => "load_with_xml"
```

Using traits (all methods start from the underscore symbol):

```rust
use voca_rs::Voca;

"LazyLoad with XMLHttpRequest and snake_case"
._words()
// => ["Lazy", "Load", "with", "XML", "Http", "Request", "and", "snake", "case"]
.join(" ")
// => "Lazy Load with XML Http Request and snake case"
._prune(21, "")
// => "Lazy Load with XML..."
._slice(5, -2)
// => "Load with XML."
._snake_case();
// => "load_with_xml"
```

## Documentation

See the complete documentation at https://a-merezhanyi.github.io/voca_rs/

Run tests: `cargo test`<br>
Build docs: `cargo doc` -> `./target/doc/voca_rs/index.html` <br>
Build a project: `cargo build` -> `./target/debug`

## Functions

#### Case

- [camel_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/case/fn.camel_case.html)
- [capitalize](https://a-merezhanyi.github.io/voca_rs/voca_rs/case/fn.capitalize.html)
- [decapitalize](https://a-merezhanyi.github.io/voca_rs/voca_rs/case/fn.decapitalize.html)
- [kebab_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/case/fn.kebab_case.html)
- [lower_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/case/fn.lower_case.html)
- [lower_first](https://a-merezhanyi.github.io/voca_rs/voca_rs/case/fn.lower_first.html)
- [pascal_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/case/fn.pascal_case.html)
- [shouty_kebab_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/case/fn.shouty_kebab_case.html)
- [shouty_snake_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/case/fn.shouty_snake_case.html)
- [snake_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/case/fn.snake_case.html)
- [swap_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/case/fn.swap_case.html)
- [title_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/case/fn.title_case.html)
- [train_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/case/fn.train_case.html)
- [upper_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/case/fn.upper_case.html)
- [upper_first](https://a-merezhanyi.github.io/voca_rs/voca_rs/case/fn.upper_first.html)

#### Chop

- [after](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.after.html)
- [after_last](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.after_last.html)
- [before](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.before.html)
- [before_last](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.before_last.html)
- [char_at](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.char_at.html)
- [code_point_at](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.code_point_at.html)
- [first](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.first.html)
- [foreign_key](https://a-merezhanyi.github.io/voca_rs/voca_rs/case/fn.foreign_key.html)
- [grapheme_at](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.grapheme_at.html)
- [last](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.last.html)
- [max](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.max.html)
- [min](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.min.html)
- [prune](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.prune.html)
- [removeprefix](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.removeprefix.html)
- [removesuffix](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.removesuffix.html)
- [slice](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.slice.html)
- [substr](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.substr.html)
- [substring](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.substring.html)
- [truncate](https://a-merezhanyi.github.io/voca_rs/voca_rs/chop/fn.truncate.html)

#### Count

- [count](https://a-merezhanyi.github.io/voca_rs/voca_rs/count/fn.count.html)
- [count_graphemes](https://a-merezhanyi.github.io/voca_rs/voca_rs/count/fn.count_graphemes.html)
- [count_substrings](https://a-merezhanyi.github.io/voca_rs/voca_rs/count/fn.count_substrings.html)
- [count_unique_words](https://a-merezhanyi.github.io/voca_rs/voca_rs/count/fn.count_unique_words.html)
- [count_where](https://a-merezhanyi.github.io/voca_rs/voca_rs/count/fn.count_where.html)
- [count_words](https://a-merezhanyi.github.io/voca_rs/voca_rs/count/fn.count_words.html)

#### Escape

- [escape_html](https://a-merezhanyi.github.io/voca_rs/voca_rs/escape/fn.escape_html.html)
- [escape_regexp](https://a-merezhanyi.github.io/voca_rs/voca_rs/escape/fn.escape_regexp.html)
- [unescape_html](https://a-merezhanyi.github.io/voca_rs/voca_rs/escape/fn.unescape_html.html)

#### Index

- [index_all](https://a-merezhanyi.github.io/voca_rs/voca_rs/index/fn.index_all.html)
- [index_of](https://a-merezhanyi.github.io/voca_rs/voca_rs/index/fn.index_of.html)
- [last_index_of](https://a-merezhanyi.github.io/voca_rs/voca_rs/index/fn.last_index_of.html)
- [search](https://a-merezhanyi.github.io/voca_rs/voca_rs/index/fn.search.html)

#### Manipulate

- [expand_spaces](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.expand_spaces.html)
- [expand_tabs](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.expand_tabs.html)
- [insert](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.insert.html)
- [latinise](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.latinise.html)
- [pad](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.pad.html)
- [pad_left](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.pad_left.html)
- [pad_right](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.pad_right.html)
- [repeat](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.repeat.html)
- [replace](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.replace.html)
- [replace_all](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.replace_all.html)
- [reverse](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.reverse.html)
- [reverse_grapheme](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.reverse_grapheme.html)
- [slugify](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.slugify.html)
- [splice](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.splice.html)
- [tr](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.tr.html)
- [trim](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.trim.html)
- [trim_left](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.trim_left.html)
- [trim_right](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.trim_right.html)
- [word_wrap](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.word_wrap.html)
- [zfill](https://a-merezhanyi.github.io/voca_rs/voca_rs/manipulate/fn.zfill.html)

#### Query

- [ends_with](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.ends_with.html)
- [includes](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.includes.html)
- [is_alpha](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_alpha.html)
- [is_alphadigit](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_alphadigit.html)
- [is_blank](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_blank.html)
- [is_camel_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_camel_case.html)
- [is_capitalize](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_capitalize.html)
- [is_decapitalize](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_decapitalize.html)
- [is_digit](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_digit.html)
- [is_empty](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_empty.html)
- [is_foreign_key](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_foreign_key.html)
- [is_lowercase](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_lowercase.html)
- [is_lower_first](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_lower_first.html)
- [is_kebab_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_kebab_case.html)
- [is_shouty_kebab_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_shouty_kebab_case.html)
- [is_numeric](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_numeric.html)
- [is_pascal_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_pascal_case.html)
- [is_snake_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_snake_case.html)
- [is_shouty_snake_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_shouty_snake_case.html)
- [is_title](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_title.html)
- [is_train_case](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_train_case.html)
- [is_uppercase](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_uppercase.html)
- [is_upper_first](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.is_upper_first.html)
- [matches](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.matches.html)
- [query](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.query.html)
- [starts_with](https://a-merezhanyi.github.io/voca_rs/voca_rs/query/fn.starts_with.html)

#### Split

- [chars](https://a-merezhanyi.github.io/voca_rs/voca_rs/split/fn.chars.html)
- [code_points](https://a-merezhanyi.github.io/voca_rs/voca_rs/split/fn.code_points.html)
- [graphemes](https://a-merezhanyi.github.io/voca_rs/voca_rs/split/fn.graphemes.html)
- [split](https://a-merezhanyi.github.io/voca_rs/voca_rs/split/fn.split.html)
- [words](https://a-merezhanyi.github.io/voca_rs/voca_rs/split/fn.words.html)

#### Strip

- [strip_bom](https://a-merezhanyi.github.io/voca_rs/voca_rs/strip/fn.strip_bom.html)
- [strip_tags](https://a-merezhanyi.github.io/voca_rs/voca_rs/strip/fn.strip_tags.html)

#### Utils

- [ASCII_LETTERS](https://a-merezhanyi.github.io/voca_rs/voca_rs/utils/constant.ASCII_LETTERS.html)
- [ASCII_LOWERCASE](https://a-merezhanyi.github.io/voca_rs/voca_rs/utils/constant.ASCII_LOWERCASE.html)
- [ASCII_UPPERCASE](https://a-merezhanyi.github.io/voca_rs/voca_rs/utils/constant.ASCII_UPPERCASE.html)
- [DIGITS](https://a-merezhanyi.github.io/voca_rs/voca_rs/utils/constant.DIGITS.html)
- [HEXDIGITS](https://a-merezhanyi.github.io/voca_rs/voca_rs/utils/constant.HEXDIGITS.html)
- [OCTDIGITS](https://a-merezhanyi.github.io/voca_rs/voca_rs/utils/constant.OCTDIGITS.html)
- [PUNCTUATION](https://a-merezhanyi.github.io/voca_rs/voca_rs/utils/constant.PUNCTUATION.html)
- [PRINTABLE](https://a-merezhanyi.github.io/voca_rs/voca_rs/utils/constant.PRINTABLE.html)
- [WHITESPACE](https://a-merezhanyi.github.io/voca_rs/voca_rs/utils/constant.WHITESPACE.html)
- [VERSION](https://a-merezhanyi.github.io/voca_rs/voca_rs/utils/constant.VERSION.html)

## Roadmap

- Ensure respecting unicode string for each method. Update each function's description clearly stating whether it works with UTF or ASCII.
- Possible refactoring: all `position` indexes covert to zero-based and add a comment to each doc
- Change all inner arguments to Enums (instead `string` or `bool`)
- To ensure that all regexp expressions are wrapped by `lazy_static`

#### Chop

- limit_words (from "words") - [link](https://docs.grafite.ca/utilities/helpers/)
- deconstantize - [link](https://docs.rs/Inflector/0.11.4/inflector/string/deconstantize/fn.deconstantize.html)
- demodulize - [link](https://docs.rs/Inflector/0.11.4/inflector/string/demodulize/fn.demodulize.html)

### Manipulate

- finish - [link](https://docs.grafite.ca/utilities/helpers/)
- start - [link](https://docs.grafite.ca/utilities/helpers/)
- deordinalize - [link](https://docs.rs/Inflector/0.11.4/inflector/numbers/deordinalize/fn.deordinalize.html)
- pluralize - [link](https://docs.rs/Inflector/0.11.4/inflector/string/pluralize/fn.to_plural.html)
- singularize - [link](https://docs.rs/Inflector/0.11.4/inflector/string/singularize/fn.to_singular.html)
- ordinalize - [link](https://docs.rs/Inflector/0.11.4/inflector/numbers/ordinalize/index.html)

## Copyright

Coded by Anatol Marezhanyi [@a_merezhanyi](https://twitter.com/a_merezhanyi)

## License

Licensed under [MIT License](LICENSE)
