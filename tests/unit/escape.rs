//! voca_rs::escape testing
use voca_rs::Voca;

#[test]
fn escape_html() {
    assert_eq!(voca_rs::escape::escape_html(""), "");
    assert_eq!(
        voca_rs::escape::escape_html("<>&\"'`"),
        "&lt;&gt;&amp;&quot;&#x27;&#x60;"
    );
    assert_eq!(
        voca_rs::escape::escape_html(voca_rs::utils::PUNCTUATION),
        "!&quot;#$%&amp;&#x27;()*+,-./:;&lt;=&gt;?@[\\]^_&#x60;{|}~"
    );
    assert_eq!(
        voca_rs::escape::escape_html("<p>wonderful world</p>"),
        "&lt;p&gt;wonderful world&lt;/p&gt;"
    );
    assert_eq!(voca_rs::escape::escape_html("<span>"), "&lt;span&gt;");
    assert_eq!(
        voca_rs::escape::escape_html("<p>wonderful<span>world<span/></p>"),
        "&lt;p&gt;wonderful&lt;span&gt;world&lt;span/&gt;&lt;/p&gt;"
    );
}
#[test]
fn escape_html_me() {
    assert_eq!(
        "<>&\"'`".escape_html(),
        "&lt;&gt;&amp;&quot;&#x27;&#x60;"
    );
}
#[test]
fn escape_regexp() {
    assert_eq!(voca_rs::escape::escape_regexp(""), "");
    assert_eq!(
        voca_rs::escape::escape_regexp("(hours)[minutes]{seconds}"),
        "\\(hours\\)\\[minutes\\]\\{seconds\\}"
    );
    assert_eq!(
        voca_rs::escape::escape_regexp("-[]/{}()*+?.\\^$|"),
        "\\-\\[\\]\\/\\{\\}\\(\\)\\*\\+\\?\\.\\\\\\^\\$\\|"
    );
}
#[test]
fn escape_regexp_me() {
    assert_eq!(
        "(hours)[minutes]{seconds}".escape_regexp(),
        "\\(hours\\)\\[minutes\\]\\{seconds\\}"
    );
}
#[test]
fn unescape_html() {
    assert_eq!(voca_rs::escape::escape_html(""), "");
    assert_eq!(
        voca_rs::escape::unescape_html("&lt;&gt;&amp;&quot;&#x27;&#x60;"),
        "<>&\"'`"
    );
    assert_eq!(
        voca_rs::escape::unescape_html("!&quot;#$%&amp;&#x27;()*+,-./:;&lt;=&gt;?@[\\]^_&#x60;{|}~"),
        voca_rs::utils::PUNCTUATION
    );
    assert_eq!(
        voca_rs::escape::unescape_html("&lt;p&gt;wonderful world&lt;/p&gt;"),
        "<p>wonderful world</p>"
    );
    assert_eq!(voca_rs::escape::unescape_html("&lt;span&gt;"), "<span>");
    assert_eq!(
        voca_rs::escape::unescape_html("&lt;p&gt;wonderful&lt;span&gt;world&lt;span/&gt;&lt;/p&gt;"),
        "<p>wonderful<span>world<span/></p>"
    );
}
#[test]
fn unescape_html_me() {
    assert_eq!(
        "&lt;&gt;&amp;&quot;&#x27;&#x60;".unescape_html(),
        "<>&\"'`"
    );
}