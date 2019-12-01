// Readme's example testing

#[test]
fn example_functions() {
    let input_string = "LazyLoad with XMLHttpRequest and snake_case";
    let string_in_words = voca_rs::split::words(&input_string);
    // => ["Lazy", "Load", "with", "XML", "Http", "Request", "and", "snake", "case"]
    assert_eq!(
        string_in_words,
        ["Lazy", "Load", "with", "XML", "Http", "Request", "and", "snake", "case"]
    );
    let words_in_string = &string_in_words.join(" ");
    // => "Lazy Load with XML Http Request and snake case"
    assert_eq!(
        words_in_string,
        "Lazy Load with XML Http Request and snake case"
    );
    let truncated_string = voca_rs::chop::prune(&words_in_string, 21, "");
    // => "Lazy Load with XML..."
    assert_eq!(truncated_string, "Lazy Load with XML...");
    let sliced_string = voca_rs::chop::slice(&truncated_string, 5, -2);
    // => "Load with XML."
    assert_eq!(sliced_string, "Load with XML.");
    let snaked_string = voca_rs::case::snake_case(&sliced_string);
    // => "load_with_xml"
    assert_eq!(snaked_string, "load_with_xml");
}

#[test]
fn example_traits() {
    use voca_rs::Voca;

    // Test #1
    let input_string1 = "LazyLoad with XMLHttpRequest and snake_case";
    let expected_result1 = [
        "Lazy", "Load", "with", "XML", "Http", "Request", "and", "snake", "case",
    ];
    assert_eq!(input_string1._words(), expected_result1);

    // Test #2
    let input_string2 = "LazyLoad with XMLHttpRequest and snake_case"._words();
    let expected_result2 = "Lazy Load with XML Http Request and snake case";
    assert_eq!(input_string2.join(" "), expected_result2);

    // Test #3
    let input_string3 = "LazyLoad with XMLHttpRequest and snake_case"
        ._words()
        .join(" ");
    let expected_string3 = "Lazy Load with XML...";
    assert_eq!(input_string3._prune(21, ""), expected_string3);

    // Test #4
    let input_string4 = "LazyLoad with XMLHttpRequest and snake_case"
        ._words()
        .join(" ")
        ._prune(21, "");
    let expected_string4 = "Load with XML.";
    assert_eq!(input_string4._slice(5, -2), expected_string4);

    // Test #5
    let input_string5 = "LazyLoad with XMLHttpRequest and snake_case"
        ._words()
        .join(" ")
        ._prune(21, "")
        ._slice(5, -2);
    let expected_string5 = "load_with_xml";
    assert_eq!(input_string5._snake_case(), expected_string5);

    // Test #6
    let input_string6 = "LazyLoad with XMLHttpRequest and snake_case"
        ._words()
        .join(" ")
        ._prune(21, "")
        ._slice(5, -2)
        ._snake_case();
    let expected_string6 = "load_with_xml";
    assert_eq!(input_string6, expected_string6);
}
