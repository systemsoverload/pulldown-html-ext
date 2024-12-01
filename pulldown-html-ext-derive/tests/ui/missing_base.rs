use pulldown_html_ext_derive::html_writer;

struct HtmlWriterBase<W>(W);

#[html_writer]
struct MissingBase<W> {  // This should fail
    writer: W,  // Wrong field name
}

fn main() {}
