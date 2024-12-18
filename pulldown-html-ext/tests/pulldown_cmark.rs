use html_compare_rs::{assert_html_eq, presets::markdown};
use pulldown_cmark::{BrokenLink, Options, Parser};
use pulldown_html_ext::{push_html, HtmlConfig};

#[test]
fn test_script_block_single_line() {
    let original = r##"Little header

<script type="text/js">
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;
    let expected = r##"<p>Little header</p>
<script type="text/js">
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;

    let parser = Parser::new(original);
    let mut output = String::new();
    push_html(&mut output, parser, &HtmlConfig::default()).unwrap();
    assert_html_eq!(output, expected, markdown());
}

#[test]
fn test_script_block_multiline_type() {
    let original = r##"Little header

<script
type="text/js">
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;
    let expected = r##"<p>Little header</p>
<script
type="text/js">
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;

    let parser = Parser::new(original);
    let mut output = String::new();
    push_html(&mut output, parser, &HtmlConfig::default()).unwrap();
    assert_html_eq!(output, expected, markdown());
}

#[test]
fn test_processing_instruction() {
    let original = r##"Little header

<?
<div></div>
<p>Useless</p>
?>"##;
    let expected = r##"<p>Little header</p>
<?
<div></div>
<p>Useless</p>
?>"##;

    let parser = Parser::new(original);
    let mut output = String::new();
    push_html(&mut output, parser, &HtmlConfig::default()).unwrap();
    assert_html_eq!(output, expected, markdown());
}

#[test]
fn test_html_comment() {
    let original = r##"Little header

<!--
<div></div>
<p>Useless</p>
-->"##;
    let expected = r##"<p>Little header</p>
<!--
<div></div>
<p>Useless</p>
-->"##;

    let parser = Parser::new(original);
    let mut output = String::new();
    push_html(&mut output, parser, &HtmlConfig::default()).unwrap();
    assert_html_eq!(output, expected, markdown());
}

#[test]
fn test_cdata_section() {
    let original = r##"Little header

<![CDATA[
<div></div>
<p>Useless</p>
]]>"##;
    let expected = r##"<p>Little header</p>
<![CDATA[
<div></div>
<p>Useless</p>
]]>"##;

    let parser = Parser::new(original);
    let mut output = String::new();
    push_html(&mut output, parser, &HtmlConfig::default()).unwrap();
    assert_html_eq!(output, expected, markdown());
}

#[test]
fn test_declaration() {
    let original = r##"Little header

<!X
Some things are here...
>"##;
    let expected = r##"<p>Little header</p>
<!X
Some things are here...
>"##;

    let parser = Parser::new(original);
    let mut output = String::new();
    push_html(&mut output, parser, &HtmlConfig::default()).unwrap();
    assert_html_eq!(output, expected, markdown());
}

#[test]
fn test_header_with_script() {
    let original = r##"Little header
-----------

<script>
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;
    let expected = r##"<h2 id="heading-2">Little header</h2>
<script>
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;

    let parser = Parser::new(original);
    let mut output = String::new();
    push_html(&mut output, parser, &HtmlConfig::default()).unwrap();
    assert_html_eq!(output, expected, markdown());
}

#[test]
fn test_table() {
    let original = "A | B\n---|---\nfoo | bar";
    let expected = r##"<table>
<thead><tr><th>A</th><th>B</th></tr></thead>
<tbody><tr><td>foo</td><td>bar</td></tr></tbody>
</table>"##;

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(original, opts);
    let mut output = String::new();
    push_html(&mut output, parser, &HtmlConfig::default()).unwrap();
    assert_html_eq!(output, expected, markdown());
}

#[test]
fn test_horizontal_rule_dash() {
    let original = "---";
    let expected = "<hr>";

    let parser = Parser::new(original);
    let mut output = String::new();
    push_html(&mut output, parser, &HtmlConfig::default()).unwrap();
    assert_html_eq!(output, expected, markdown());
}

#[test]
fn test_horizontal_rule_asterisk() {
    let original = "* * *";
    let expected = "<hr>";

    let parser = Parser::new(original);
    let mut output = String::new();
    push_html(&mut output, parser, &HtmlConfig::default()).unwrap();
    assert_html_eq!(output, expected, markdown());
}

#[test]
fn test_strikethrough_disabled() {
    let original = "hi ~~no~~";
    let expected = "<p>hi ~~no~~</p>";

    let parser = Parser::new(original);
    let mut output = String::new();
    push_html(&mut output, parser, &HtmlConfig::default()).unwrap();
    assert_html_eq!(output, expected, markdown());
}

#[test]
fn test_broken_link_callback() {
    let original = r##"[foo],
[bar],
[baz],

   [baz]: https://example.org
"##;
    let expected = r##"<p><a href="https://replaced.example.org" title="some title">foo</a>,
[bar],
<a href="https://example.org">baz</a>,</p>"##;

    let mut callback = |broken_link: BrokenLink| {
        if &*broken_link.reference == "foo" || &*broken_link.reference == "baz" {
            Some(("https://replaced.example.org".into(), "some title".into()))
        } else {
            None
        }
    };

    let parser =
        Parser::new_with_broken_link_callback(original, Options::empty(), Some(&mut callback));
    let mut output = String::new();
    let mut config = HtmlConfig::default();
    config.html.break_on_newline = false;
    config.elements.links.nofollow_external = false;
    config.elements.links.open_external_blank = false;
    push_html(&mut output, parser, &config).unwrap();
    assert_html_eq!(output, expected, markdown());
}

#[test]
fn test_code_with_newlines() {
    for original in ["`\n `x", "` \n`x"] {
        let expected = "<p><code>  </code>x</p>";
        let parser = Parser::new(original);
        let mut output = String::new();
        push_html(&mut output, parser, &HtmlConfig::default()).unwrap();
        assert_html_eq!(output, expected, markdown());
    }
}

#[test]
fn test_code_with_newlines_at_boundaries() {
    let original = "`\nx\n`x";
    let expected = "<p><code>x</code>x</p>";

    let parser = Parser::new(original);
    let mut output = String::new();
    push_html(&mut output, parser, &HtmlConfig::default()).unwrap();
    assert_html_eq!(output, expected, markdown());
}

#[test]
fn test_trim_whitespace_at_paragraph_end() {
    let original = "one\ntwo \t";
    let expected = "<p>one\ntwo</p>";

    let parser = Parser::new(original);
    let mut output = String::new();
    let mut config = HtmlConfig::default();
    config.html.break_on_newline = false;
    push_html(&mut output, parser, &config).unwrap();
    assert_html_eq!(output, expected, markdown());
}

#[test]
fn test_code_with_internal_newlines() {
    for original in ["`\nx \ny\n`x", "`x \ny`x", "`x\n y`x"] {
        let expected = "<p><code>x  y</code>x</p>";
        let parser = Parser::new(original);
        let mut output = String::new();
        push_html(&mut output, parser, &HtmlConfig::default()).unwrap();
        assert_html_eq!(output, expected, markdown());
    }
}

#[test]
fn test_trim_whitespace_and_newline_at_paragraph_end() {
    let expected = "<p>one\ntwo</p>";
    let mut config = HtmlConfig::default();
    config.html.break_on_newline = false;

    let parser = Parser::new("one\ntwo \t\n");
    let mut output = String::new();
    push_html(&mut output, parser, &config).unwrap();
    assert_html_eq!(output, expected, markdown());
}

#[test]
fn test_trim_space_before_newline_at_paragraph_end() {
    let original = "one\ntwo \n";
    let expected = "<p>one\ntwo</p>";

    let parser = Parser::new(original);
    let mut output = String::new();
    let mut config = HtmlConfig::default();
    config.html.break_on_newline = false;
    push_html(&mut output, parser, &config).unwrap();
    assert_html_eq!(output, expected, markdown());
}

#[test]
fn test_trim_space_before_soft_break() {
    let original = "one \ntwo";
    let expected = "<p>one\ntwo</p>";

    let parser = Parser::new(original);
    let mut output = String::new();
    let mut config = HtmlConfig::default();
    config.html.break_on_newline = false;
    push_html(&mut output, parser, &config).unwrap();
    assert_html_eq!(output, expected, markdown());
}
