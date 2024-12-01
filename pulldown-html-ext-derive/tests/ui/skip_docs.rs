use pulldown_cmark_escape::StrWrite;
use pulldown_html_ext_derive::html_writer;

// Mock types needed for compilation
#[derive(Debug)]
struct HtmlWriterBase<W>(W);
trait HtmlWriter<W: StrWrite> {}

#[html_writer(skip_docs)]
#[derive(Debug)]
struct NoDocsWriter<W> {
    base: HtmlWriterBase<W>,
}

fn main() {}
