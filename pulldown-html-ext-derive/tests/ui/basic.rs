use pulldown_cmark_escape::StrWrite;
use pulldown_html_ext_derive::html_writer;

// Mock types needed for compilation
struct HtmlWriterBase<W>(W);
trait HtmlWriter<W: StrWrite> {}

#[html_writer]
struct BasicWriter<W> {
    base: HtmlWriterBase<W>,
}

fn main() {}
