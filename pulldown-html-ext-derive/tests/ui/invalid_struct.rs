use pulldown_html_ext_derive::html_writer;

struct HtmlWriterBase<W>(W);

#[html_writer]
enum InvalidEnum {  // This should fail
    Variant(HtmlWriterBase<String>),
}

fn main() {}
