//! Trait implementation testing

use voca_rs::*;

#[test]
fn impl_test() {
    let mut string;
    let mut exp;

    // case::camel_case
    string = "The World - IS Yours";
    exp = "theWorldIsYours";
    assert_eq!(String::from(string).camel_case(), exp);
    assert_eq!(string.camel_case(), exp);
    
    // case::capitalize
    string = "The World IS YourS";
    exp = "The world is yours";
    assert_eq!(String::from(string).capitalize(true), exp);
    assert_eq!(string.capitalize(true), exp);
    
    // case::decapitalize
    string = "The World IS YourS";
    exp = "the world is yours";
    assert_eq!(String::from(string).decapitalize(true), exp);
    assert_eq!(string.decapitalize(true), exp);
    
    // assert!("birdFlight".is_camel_case());
}