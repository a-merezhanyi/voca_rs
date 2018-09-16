# voca_rs [![Build Status](https://travis-ci.org/e1r0nd/voca_rs.svg?branch=master)](https://travis-ci.org/e1r0nd/voca_rs) [![GitHub version](https://badge.fury.io/gh/e1r0nd%2Fvoca_rs.svg)](https://badge.fury.io/gh/e1r0nd%2Fvoca_rs) [![codecov](https://codecov.io/gh/e1r0nd/voca_rs/branch/master/graph/badge.svg)](https://codecov.io/gh/e1r0nd/voca_rs) [![license](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

**Voca_rs is a Rust library for manipulating strings.**

Voca_rs is inspired by [Voca.js](https://vocajs.com/) and [string.py](https://docs.python.org/3.4/library/string.html)

## TL;DR

```rust
use voca_rs::*;

let input_string = "LazyLoad with XMLHttpRequest and snake_case";
let string_in_words = split::words(&input_string);
// => ["Lazy", "Load", "with", "XML", "Http", "Request", "and", "snake", "case"]
let snake_string = case::snake_case(&string_in_words[3..6].join(" "));
// => "xml_http_request"
```

## Documentation

See the complete documentation at https://e1r0nd.github.io/voca_rs/

Run test: `cargo test`<br>
Build docs: `cargo doc` -> `./target/doc/voca_rs/index.html` <br>
Build project: `cargo build` -> `./target/debug`<br>
Publish project: `git push`

## Roadmap

#### Case

- [x] [camel_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.camel_case.html)
- [x] [capitalize](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.capitalize.html)
- [x] [decapitalize](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.decapitalize.html)
- [x] [kebab_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.kebab_case.html)
- [x] [lower_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.lower_case.html)
- [x] [pascal_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.pascal_case.html)
- [x] [shouty_kebab_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.shouty_kebab_case.html)
- [x] [shouty_snake_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.shouty_snake_case.html)
- [x] [snake_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.snake_case.html)
- [x] [swap_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.swap_case.html)
- [x] [title_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.title_case.html)
- [x] [upper_case](https://e1r0nd.github.io/voca_rs/voca_rs/case/fn.upper_case.html)

#### Chop

- [ ] charAt
- [ ] codePointAt
- [ ] first
- [ ] graphemeAt
- [ ] last
- [ ] prune
- [ ] slice
- [ ] substr
- [ ] substring
- [ ] truncate

#### Count

- [ ] count
- [ ] countGraphemes
- [ ] countSubstrings
- [ ] countWhere
- [ ] countWords

#### Escape

- [ ] escapeHtml
- [ ] escapeRegExp
- [ ] unescapeHtml

#### Format

- [ ] format - [link](https://docs.python.org/3.4/library/string.html#string.Formatter.format)
- [ ] vformat - [link](https://docs.python.org/3.4/library/string.html#string.Formatter.vformat)
- [ ] parse - [link](https://docs.python.org/3.4/library/string.html#string.Formatter.parse)
- [ ] get_field - [link](https://docs.python.org/3.4/library/string.html#string.Formatter.get_field)
- [ ] get_value - [link](https://docs.python.org/3.4/library/string.html#string.Formatter.get_value)
- [ ] check_unused_args - [link](https://docs.python.org/3.4/library/string.html#string.Formatter.check_unused_args)
- [ ] format_field - [link](https://docs.python.org/3.4/library/string.html#string.Formatter.format_field)
- [ ] convert_field - [link](https://docs.python.org/3.4/library/string.html#string.Formatter.convert_field)
- [ ] substitute - [link](https://docs.python.org/3.4/library/string.html#string.Template.substitute)
- [ ] safe_substitute - [link](https://docs.python.org/3.4/library/string.html#string.Template.safe_substitute)
- [ ] sprintf - [link](https://vocajs.com/#sprintf)
- [ ] vprintf - [link](https://vocajs.com/#vprintf)

#### Index

- [ ] indexOf
- [ ] lastIndexOf
- [ ] search

#### Manipulate

- [ ] insert
- [ ] latinise
- [ ] pad
- [ ] padLeft
- [ ] padRight
- [ ] repeat
- [ ] replace
- [ ] replaceAll
- [ ] reverse
- [ ] reverseGrapheme
- [ ] slugify
- [ ] splice
- [ ] tr
- [ ] trim
- [ ] trimLeft
- [ ] trimRight
- [ ] wordWrap

#### Query

- [x] [ends_with](https://e1r0nd.github.io/voca_rs/voca_rs/query/fn.ends_with.html)
- [x] [includes](https://e1r0nd.github.io/voca_rs/voca_rs/query/fn.includes.html)
- [ ] isAlpha
- [ ] isAlphaDigit
- [ ] isBlank
- [ ] isDigit
- [ ] isEmpty
- [ ] isLowerCase
- [ ] isNumeric
- [ ] isString
- [ ] isUpperCase
- [ ] matches
- [x] [starts_with](https://e1r0nd.github.io/voca_rs/voca_rs/query/fn.starts_with.html)

#### Split

- [x] [chars](https://e1r0nd.github.io/voca_rs/voca_rs/split/fn.chars.html)
- [ ] codePoints
- [x] [graphemes](https://e1r0nd.github.io/voca_rs/voca_rs/split/fn.graphemes.html)
- [x] [split](https://e1r0nd.github.io/voca_rs/voca_rs/split/fn.split.html)
- [x] [words](https://e1r0nd.github.io/voca_rs/voca_rs/split/fn.words.html)

#### Strip

- [ ] stripBom
- [ ] stripTags

#### Utils

- [x] [ASCII_LETTERS](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.ASCII_LETTERS.html)
- [x] [ASCII_LOWERCASE](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.ASCII_LOWERCASE.html)
- [x] [ASCII_UPPERCASE](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.ASCII_UPPERCASE.html)
- [x] [DIGITS](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.DIGITS.html)
- [x] [HEXDIGITS](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.HEXDIGITS.html)
- [x] [OCTDIGITS](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.OCTDIGITS.html)
- [x] [PUNCTUATION](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.PUNCTUATION.html)
- [ ] printable - [link](https://docs.python.org/3.4/library/string.html#string.printable)
- [x] [WHITESPACE](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.WHITESPACE.html)
- [ ] noConflict
- [x] [VERSION](https://e1r0nd.github.io/voca_rs/voca_rs/utils/constant.VERSION.html)

## Copyright

coded by Anatol Marezhanyi [@e1r0nd_crg](https://twitter.com/e1r0nd_crg)

https://linkedin.com/in/merezhany/<br>
https://www.youtube.com/c/AnatolMerezhanyi

## License

Licensed under [MIT License](LICENSE)
