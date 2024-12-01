use pulldown_html_ext_derive::html_writer;

struct HtmlWriterBase<W>(W);

#[html_writer]
struct WrongBaseType<W> {  // This should fail
    base: String,  // Wrong type
}

fn main() {}
