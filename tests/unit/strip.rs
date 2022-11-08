//! voca_rs::strip testing
use voca_rs::Voca;

#[test]
fn strip_bom() {
    assert_eq!(voca_rs::strip::strip_bom(""), "");
    assert_eq!(voca_rs::strip::strip_bom("\u{FEFF}"), "");
    assert_eq!(
        voca_rs::strip::strip_bom(voca_rs::utils::PRINTABLE),
        voca_rs::utils::PRINTABLE
    );
    assert_eq!(
        voca_rs::strip::strip_bom("\u{FEFF}summertime sadness"),
        "summertime sadness"
    );
    assert_eq!(
        voca_rs::strip::strip_bom("\\u{FEFF}summertime sadness"),
        "\\u{FEFF}summertime sadness"
    );
    assert_eq!(
        voca_rs::strip::strip_bom("summertime sadness"),
        "summertime sadness"
    );
}
#[test]
fn _strip_bom() {
    assert_eq!(
        "\u{FEFF}summertime sadness"._strip_bom(),
        "summertime sadness"
    );
}
#[test]
fn strip_tags_general_tests() {
    // should not modify a string without tags
    assert_eq!(voca_rs::strip::strip_tags(""), "");
    assert_eq!(voca_rs::strip::strip_tags("Hello world!"), "Hello world!");
    assert_eq!(voca_rs::strip::strip_tags("  "), "  ");
    // should strip tags
    assert_eq!(
        voca_rs::strip::strip_tags("<span><a href=\"#\">Summer</a> is nice</span>"),
        "Summer is nice"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<b>Hello world!</b>"),
        "Hello world!"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<span class=\"italic\"><b>Hello world!</b></span>"),
        "Hello world!"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<span class='<italic>'>Hello world!</span>"),
        "Hello world!"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<script language=\"PHP\"> echo hello </script>"),
        " echo hello "
    );
    // should strip tags which attributes contain < or >
    assert_eq!(
        voca_rs::strip::strip_tags("hello <img title='>_<'> world"),
        "hello  world"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("hello <img title=\"<\"> world"),
        "hello  world"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("hello <img title=\"<foo/> <'bar'\"> world"),
        "hello  world"
    );
    // should strip tags on multiple lines
    assert_eq!(
        voca_rs::strip::strip_tags("This's a string with quotes:</html>\n\"strings in double quote\";\n'strings in single quote\';\n<html>this\\line is single quoted /with\\slashes"),
        "This\'s a string with quotes:\n\"strings in double quote\";\n\'strings in single quote\';\nthis\\line is single quoted /with\\slashes"
    );
    // should strip comments and doctype
    assert_eq!(
        voca_rs::strip::strip_tags("<html><!-- COMMENT --></html>"),
        ""
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<b>Hello world!</b><!-- Just some information -->"),
        "Hello world!"
    );
    assert_eq!(
        voca_rs::strip::strip_tags(
            "<span class=\"italic\">Hello world!<!-- Just some information --></span>"
        ),
        "Hello world!"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<!-- Small<>comment --><span class=\"italic\"><!-- Just some information --><b>Hello world!</b></span>"),
        "Hello world!"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<!doctype html><span class=\"italic\"><!-- Just some information --><b>Hello world!</b></span>"),
        "Hello world!"
    );
}
#[test]
fn strip_tags_user_tests() {
    assert_eq!(
        voca_rs::strip::strip_tags("<span style=\"color: rgb(51, 51, 51); font-family: \" microsoft=\"\" yahei=\"\" stheiti=\"\" wenquanyi=\"\" micro=\"\" hei=\"\" simsun=\"\" sans-serif=\"\" font-size:=\"\" 16px=\"\">】มีมี่’ เด็กสาวที่นอนไม่ค่อยหลับเนื่องจากกลัวผี ขี้เหงา และอะไรหลายๆ อย่างทำให้เธอมึนได้โล่เพราะไม่ค่อยได้นอน การที่เธอ นอนไม่หลับทำให้เธอได้เจอกับ ‘ดีเจไททัน’ แห่งคลื่น 99.99 MHzเขาจัดรายการในช่วง Midnight Fantasy ตีสามถึงตีห้า และมีมี่ก็เป็นผู้ฟังเพียงคนเดียวของเขาจากที่ตอนแรกเธอฟังดีเจไททันเพื่อช่วยปลอบประโลมการที่เธอต้องมาอยู่หอเพียงลำพัง แต่ไปๆ มาๆกลับกลายเป็นว่าเธออยู่รอฟังเขาทุกคืนทำให้เธอไปเรียนแบบมึนๆ จนบังเอิญไปนอนหลับซบ ‘ธรรม’ผู้ชายจอมกวนที่บังเอิญมานอนให้เธอซบ! จนอาจารย์สั่งให้ไปทำรายงานคู่กัน และนั่นก็เป็นที่มาของการที่เธอเริ่มไม่แน่ใจแล้วว่าเธอปลื้มดีเจไททัน หรือแอบหวั่นไหวกับนายจอมกวนคนนี้กันแน่</span><br />"),
        "】มีมี่’ เด็กสาวที่นอนไม่ค่อยหลับเนื่องจากกลัวผี ขี้เหงา และอะไรหลายๆ อย่างทำให้เธอมึนได้โล่เพราะไม่ค่อยได้นอน การที่เธอ นอนไม่หลับทำให้เธอได้เจอกับ ‘ดีเจไททัน’ แห่งคลื่น 99.99 MHzเขาจัดรายการในช่วง Midnight Fantasy ตีสามถึงตีห้า และมีมี่ก็เป็นผู้ฟังเพียงคนเดียวของเขาจากที่ตอนแรกเธอฟังดีเจไททันเพื่อช่วยปลอบประโลมการที่เธอต้องมาอยู่หอเพียงลำพัง แต่ไปๆ มาๆกลับกลายเป็นว่าเธออยู่รอฟังเขาทุกคืนทำให้เธอไปเรียนแบบมึนๆ จนบังเอิญไปนอนหลับซบ ‘ธรรม’ผู้ชายจอมกวนที่บังเอิญมานอนให้เธอซบ! จนอาจารย์สั่งให้ไปทำรายงานคู่กัน และนั่นก็เป็นที่มาของการที่เธอเริ่มไม่แน่ใจแล้วว่าเธอปลื้มดีเจไททัน หรือแอบหวั่นไหวกับนายจอมกวนคนนี้กันแน่"
    );
}
#[test]
fn strip_tags_special_tests() {
    // should treat especially broken or invalid tags
    assert_eq!(voca_rs::strip::strip_tags("< html >"), "< html >");
    assert_eq!(voca_rs::strip::strip_tags("<<>>"), "");
    assert_eq!(
        voca_rs::strip::strip_tags("<a.>HtMl text</.a>"),
        "HtMl text"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<abc>hello</abc> \t\tworld... <ppp>strip_tags_test</ppp>"),
        "hello \t\tworld... strip_tags_test"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<html><b>hello</b><p>world</p></html>"),
        "helloworld"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<span class=\"italic\"><b>He>llo</b> < world!</span>"),
        "He>llo < world!"
    );
    // should handle unicode
    assert_eq!(
        voca_rs::strip::strip_tags("<SCRIPT>Ω≈ç≈≈Ω</SCRIPT>"),
        "Ω≈ç≈≈Ω"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<SCRIPT a=\"blah\">片仮名平仮名</SCRIPT>"),
        "片仮名平仮名"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<!-- testing --><a>text here</a>"),
        "text here"
    );
}
#[test]
fn strip_tags_xss_tests() {
    // should strip potential xss tags', function() {
    // @see https://www.owasp.org/index.php/XSS_Filter_Evasion_Cheat_Sheet
    assert_eq!(
        voca_rs::strip::strip_tags(
            "<img src=\"data:image/gif;base64,R0lGODlhAQABAIAAAP///wAAACwAAAAAA‌\u{200B}QABAAACAkQBADs=\"
            onload=\"$.getScript('evil.js');1<2>3\">"
        ),
        ""
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<script>evil();</script>"),
        "evil();"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<SCRIPT SRC=http://xss.rocks/xss.js></SCRIPT>"),
        ""
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<IMG \"\"\"><SCRIPT>alert(\"XSS\")</SCRIPT>\">"),
        ""
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<SCRIPT/XSS SRC=\"http://xss.rocks/xss.js\"></SCRIPT>"),
        ""
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<BODY onload!#$%&()*~+-_.,:;?@[/|\\]^`=alert(\"XSS\")>"),
        ""
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<SCRIPT/SRC=\"http://xss.rocks/xss.js\"></SCRIPT>"),
        ""
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<<SCRIPT>alert(\"XSS\");//<</SCRIPT>"),
        ""
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<SCRIPT SRC=http://xss.rocks/xss.js?< B >"),
        ""
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<SCRIPT SRC=//xss.rocks/.j>"),
        ""
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<IMG SRC=\"javascript:alert(\'XSS\')\""),
        ""
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<SCRIPT a=\">\" SRC=\"httx://xss.rocks/xss.js\"></SCRIPT>"),
        ""
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<SCRIPT =\">\" SRC=\"httx://xss.rocks/xss.js\"></SCRIPT>"),
        ""
    );
    assert_eq!(
        voca_rs::strip::strip_tags(
            "<SCRIPT a=\">\" \'\' SRC=\"httx://xss.rocks/xss.js\"></SCRIPT>"
        ),
        ""
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<SCRIPT \"a=\'>\'\" SRC=\"httx://xss.rocks/xss.js\"></SCRIPT>"),
        ""
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<SCRIPT a=`>` SRC=\"httx://xss.rocks/xss.js\"></SCRIPT>"),
        "` SRC=\"httx://xss.rocks/xss.js\">"
    );
    assert_eq!(
        voca_rs::strip::strip_tags("<SCRIPT a=\">\'>\" SRC=\"httx://xss.rocks/xss.js\"></SCRIPT>"),
        ""
    );
    assert_eq!(voca_rs::strip::strip_tags("<SCRIPT>document.write(\"<SCRI\");</SCRIPT>PT SRC=\"httx://xss.rocks/xss.js\"></SCRIPT>"), "document.write(\"");
}
#[test]
fn _strip_tags() {
    assert_eq!(
        "<span><a href=\"#\">Summer</a> is nice</span>"._strip_tags(),
        "Summer is nice"
    );
}

#[test]
fn partial_directive() {
    assert_eq!(voca_rs::strip::strip_tags("<"), "");
    assert_eq!(voca_rs::strip::strip_tags("<t"), "");
    assert_eq!(voca_rs::strip::strip_tags("</"), "");
    assert_eq!(voca_rs::strip::strip_tags("</a"), "");
    assert_eq!(voca_rs::strip::strip_tags("<!"), "");
    assert_eq!(voca_rs::strip::strip_tags("<!-"), "");
}
