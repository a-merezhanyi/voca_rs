//! Converts the `subject` to a selected case.

use split;
/// Converts the `subject` to camel case.
///
/// # Arguments
///
/// * `subject` - The string to convert to camel case.
///
/// # Example
/// ```
/// use voca_rs::*;
/// case::camel_case("bird flight");
/// // => "birdFlight"
/// case::camel_case("BirdFlight");
/// // => "birdFlight"
/// case::camel_case("-BIRD-FLIGHT-");
/// // => "birdFlight"
/// ```
pub fn camel_case(subject: &str) -> String {
    camel_and_pascal_case(&subject, TitleMode::Normal)
}

#[derive(Clone, Copy, PartialEq)]
enum TitleMode {
    Normal,
    Caps
}

fn camel_and_pascal_case(subject: &str, title_mode: TitleMode) -> String {
    return match subject.len() {
        0 => subject.to_string(),
        _ => return_string(&subject, title_mode),
    };

    fn return_string(subject: &str, title_mode: TitleMode) -> String {
        let mut res = String::with_capacity(subject.len());
        for (i, c) in split::words(subject).iter().enumerate() {
            let s = if i == 0 && title_mode == TitleMode::Normal {
                lower_case(c)
            } else {
                capitalize(c, true)
            };
            res.push_str(&s);
        }

        res
    };
}

/// Converts the first character of `subject` to upper case. If `restToLower` is `true`, convert the rest of `subject` to lower case.
///
/// # Arguments
///
/// * `subject` - The string to capitalize.
/// * `rest_to_lower` - Convert the rest of `subject` to lower case.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// case::capitalize("green", true);
/// // => "Green"
/// case::capitalize("Say Hello to ME", true);
/// // => "Say hello to me"
/// ```
pub fn capitalize(subject: &str, rest_to_lower: bool) -> String {
    return match subject.len() {
        0 => subject.to_string(),
        _ => return_string(&subject, rest_to_lower),
    };

    fn return_string(subject: &str, rest_to_lower: bool) -> String {
        let mut res = String::with_capacity(subject.len());
        for (i, c) in split::chars(subject).iter().enumerate() {
            let s = if i == 0 {
                c.to_uppercase()
            } else if rest_to_lower {
                c.to_lowercase()
            } else {
                c.to_string()
            };
            res.push_str(&s);
        }

        res
    };
}

/// Converts the first character of `subject` to lower case. If `restToLower` is `true`, convert the rest of `subject` to lower case.
///
/// # Arguments
///
/// * `subject` - The string to decapitalize.
/// * `rest_to_lower` - Convert the rest of `subject` to lower case.
/// # Example
///
/// ```
/// use voca_rs::*;
/// case::decapitalize("Green", true);
/// // => "green"
/// case::decapitalize("Say Hello to ME", false);
/// // => "say Hello to ME"
/// ```
pub fn decapitalize(subject: &str, rest_to_lower: bool) -> String {
    return match subject.len() {
        0 => subject.to_string(),
        _ => return_string(&subject, rest_to_lower),
    };

    fn return_string(subject: &str, rest_to_lower: bool) -> String {
        let mut res = String::with_capacity(subject.len());
        for (i, c) in split::chars(subject).iter().enumerate() {
            let s = if i == 0 || rest_to_lower {
                c.to_lowercase()
            } else {
                c.to_string()
            };
            res.push_str(&s);
        }

        res
    };
}

/// Converts the `subject` to kebab case.
///
/// # Arguments
///
/// * `subject` - The string to convert to kebab case.
///
/// # Example
/// ```
/// use voca_rs::*;
/// case::kebab_case("goodbye blue sky");
/// // => "goodbye-blue-sky"
/// case::kebab_case("GoodbyeBlueSky");
/// // => "goodbye-blue-sky"
/// case::kebab_case("-Goodbye-Blue-Sky-");
/// // => "goodbye-blue-sky"
/// ```
pub fn kebab_case(subject: &str) -> String {
    kebab_and_shouty_kebab_and_train_case(&subject, KebabMode::Normal)
}

/// Converts the `subject` to SHOUTY kebab case.
///
/// # Arguments
///
/// * `subject` - The string to convert to SHOUTY kebab case.
///
/// # Example
/// ```
/// use voca_rs::*;
/// case::shouty_kebab_case("goodbye blue sky");
/// // => "GOODBYE-BLUE-SKY"
/// case::shouty_kebab_case("GoodbyeBlueSky");
/// // => "GOODBYE-BLUE-SKY"
/// case::shouty_kebab_case("-Goodbye-Blue-Sky-");
/// // => "GOODBYE-BLUE-SKY"
/// ```
pub fn shouty_kebab_case(subject: &str) -> String {
    kebab_and_shouty_kebab_and_train_case(&subject, KebabMode::Shouty)
}

#[derive(Clone, Copy, PartialEq)]
enum KebabMode {
    Normal,
    Shouty,
    Train,
}

fn kebab_and_shouty_kebab_and_train_case(subject: &str, kebab_mode: KebabMode) -> String {
    match subject.len() {
        0 => subject.to_string(),
        _ => split::words(subject)
            .into_iter()
            .map(|c| match kebab_mode {
                KebabMode::Normal => lower_case(&c),
                KebabMode::Shouty => upper_case(&c),
                KebabMode::Train => capitalize(&c, true),
            })
            .collect::<Vec<String>>()
            .join("-"),
    }
}

/// Converts the `subject` to lower case.
///
/// # Arguments
///
/// * `subject` - The string to convert to lower case.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// case::lower_case("Green");
/// // => "green"
/// case::lower_case("Say Hello to ME");
/// // => "say hello to me"
/// ```
pub fn lower_case(subject: &str) -> String {
    match subject.len() {
        0 => subject.to_string(),
        _ => {
            let mut res = String::with_capacity(subject.len());
            for c in split::chars(subject).iter() {
                res.push_str(&c.to_lowercase());
            }
            res
        }
    }
}

/// Converts the `subject` to pascal case.
///
/// # Arguments
///
/// * `subject` - The string to convert to pascal case.
///
/// # Example
/// ```
/// use voca_rs::*;
/// case::pascal_case("bird flight");
/// // => "BirdFlight"
/// case::pascal_case("BirdFlight");
/// // => "BirdFlight"
/// case::pascal_case("-BIRD-FLIGHT-");
/// // => "BirdFlight"
/// ```
pub fn pascal_case(subject: &str) -> String {
    camel_and_pascal_case(&subject, TitleMode::Caps)
}

/// Converts the `subject` to snake case.
///
/// # Arguments
///
/// * `subject` - The string to convert to snake case.
///
/// # Example
/// ```
/// use voca_rs::*;
/// case::snake_case("learning to fly");
/// // => "learning_to_fly"
/// case::snake_case("LearningToFly");
/// // => "learning_to_fly"
/// case::snake_case("-Learning-To-Fly-");
/// // => "learning_to_fly"
/// ```
pub fn snake_case(subject: &str) -> String {
    snake_and_shouty_snake_case(&subject, false)
}

/// Converts the `subject` to SHOUTY snake case.
///
/// # Arguments
///
/// * `subject` - The string to convert to shouty snake case.
///
/// # Example
/// ```
/// use voca_rs::*;
/// case::shouty_snake_case("learning to fly");
/// // => "LEARNING_TO_FLY"
/// case::shouty_snake_case("LearningToFly");
/// // => "LEARNING_TO_FLY"
/// case::shouty_snake_case("-Learning-To-Fly-");
/// // => "LEARNING_TO_FLY"
/// ```
pub fn shouty_snake_case(subject: &str) -> String {
    snake_and_shouty_snake_case(&subject, true)
}

fn snake_and_shouty_snake_case(subject: &str, shouty: bool) -> String {
    match subject.len() {
        0 => subject.to_string(),
        _ => split::words(subject)
            .into_iter()
            .map(|c| {
                if shouty {
                    upper_case(&c)
                } else {
                    lower_case(&c)
                }
            })
            .collect::<Vec<String>>()
            .join("_"),
    }
}

/// Converts the uppercase alpha caracters of `subject` to lowercase and lowercase characters to uppercase.
///
/// # Arguments
///
/// * `subject` - The string to swap the case.
///
/// # Example
/// ```
/// use voca_rs::*;
/// case::swap_case("League of Shadows");
/// // => "lEAGUE OF sHADOWS"
/// case::swap_case("2 üBer Bees");
/// // => "2 ÜbER bEES"
/// ```
pub fn swap_case(subject: &str) -> String {
    match subject.len() {
        0 => subject.to_string(),
        _ => split::chars(subject)
            .into_iter()
            .map(|s| {
                s.chars()
                    .filter_map(|c| {
                        if c.is_lowercase() {
                            c.to_uppercase().next()
                        } else {
                            c.to_lowercase().next()
                        }
                    })
                    .collect()
            })
            .collect::<Vec<String>>()
            .join(""),
    }
}

/// Converts the `subject` to title case.
///
/// # Arguments
///
/// * `subject` - The string to convert to title case.
///
/// # Example
/// ```
/// use voca_rs::*;
/// case::title_case("bird flight");
/// // => "Bird Flight"
/// case::title_case("BirdFlight");
/// // => "Bird Flight"
/// case::title_case("-BIRD-FLIGHT-");
/// // => "Bird Flight"
/// ```
pub fn title_case(subject: &str) -> String {
    match subject.len() {
        0 => subject.to_string(),
        _ => split::words(subject)
            .into_iter()
            .map(|c| capitalize(&c, true))
            .collect::<Vec<String>>()
            .join(" "),
    }
}

/// Converts the `subject` to train case.
///
/// # Arguments
///
/// * `subject` - The string to convert to train case.
///
/// # Example
/// ```
/// use voca_rs::*;
/// case::train_case("goodbye blue sky");
/// // => "Goodbye-Blue-Sky"
/// case::train_case("GoodbyeBlueSky");
/// // => "Goodbye-Blue-Sky"
/// case::train_case("-Goodbye-Blue-Sky-");
/// // => "Goodbye-Blue-Sky"
/// ```
pub fn train_case(subject: &str) -> String {
    kebab_and_shouty_kebab_and_train_case(&subject, KebabMode::Train)
}

/// Converts the `subject` to upper case.
///
/// # Arguments
///
/// * `subject` - The string to convert to upper case.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// case::upper_case("Green");
/// // => "GREEN"
/// case::upper_case("Say Hello to ME");
/// // => "SAY HELLO TO ME"
/// ```
pub fn upper_case(subject: &str) -> String {
    match subject.len() {
        0 => subject.to_string(),
        _ => {
            let mut res = String::with_capacity(subject.len());
            for c in split::chars(subject).iter() {
                res.push_str(&c.to_uppercase());
            }
            res
        }
    }
}

/// Converts the first character of the `subject` to lower case.
///
/// # Arguments
///
/// * `subject` - The string to convert.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// case::lower_first("Fred");
/// // => "fred"
/// case::lower_first("FRED");
/// // => "fRED"
/// ```
pub fn lower_first(subject: &str) -> String {
    decapitalize(&subject, false)
}

/// Converts the first character of the `subject` to upper case.
///
/// # Arguments
///
/// * `subject` - The string to convert.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// case::upper_first("fred");
/// // => "Fred"
/// case::upper_first("FRED");
/// // => "FRED"
/// ```
pub fn upper_first(subject: &str) -> String {
    capitalize(&subject, false)
}
