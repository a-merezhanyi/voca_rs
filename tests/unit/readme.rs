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

    let snake_string = voca_rs::case::snake_case(&voca_rs::chop::slice(&words_in_string, 14, 31));
    // => "xml_http_request"
    assert_eq!(snake_string, "xml_http_request");

    let truncated_string = voca_rs::chop::prune(&words_in_string, 15, "");
    // => "Lazy Load..."
    assert_eq!(truncated_string, "Lazy Load...");
}

#[test]
fn example_traits() {
    use voca_rs::Voca;

    let mut input_string;
    let mut expected_result;
    let mut expected_string;

    input_string = "LazyLoad with XMLHttpRequest and snake_case";
    expected_result = ["Lazy", "Load", "with", "XML", "Http", "Request", "and", "snake", "case"];
    assert_eq!(input_string.words(), expected_result);

    input_string = "LazyLoad with XMLHttpRequest and snake_case".words();
    expected_string = "Lazy Load with XML Http Request and snake case";
    assert_eq!(input_string.join(" "), expected_result);

    input_string = "LazyLoad with XMLHttpRequest and snake_case".words().join(" ");
    expected_string = "Lazy Load with XML...";
    assert_eq!(input_string.prune(20, ""), expected_string);

    input_string = "LazyLoad with XMLHttpRequest and snake_case".words().join(" ").prune(20, "");
    expected_string = "Load with XML.";
    assert_eq!(input_string.slice(5, -2), expected_string);

    input_string = "LazyLoad with XMLHttpRequest and snake_case".words().join(" ").prune(20, "").slice(5, -2);
    expected_string = "load_with_xml";
    assert_eq!(input_string.snake_case(), expected_string);
}
