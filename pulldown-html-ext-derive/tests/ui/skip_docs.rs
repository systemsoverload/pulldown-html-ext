use pulldown_cmark_escape::StrWrite;
use pulldown_html_ext_derive::html_writer;

// Mock types needed for compilation
#[derive(Debug)]
struct HtmlConfig {
    // Configuration fields would go here
}

#[derive(Debug)]
struct HtmlState {
    // State fields would go here
}

#[derive(Debug)]
struct HtmlWriterBase<W> {
    writer: W,
    config: HtmlConfig,
    state: HtmlState,
}

impl<W> HtmlWriterBase<W> {
    fn get_writer(&mut self) -> &mut W {
        &mut self.writer
    }

    fn get_config(&self) -> &HtmlConfig {
        &self.config
    }

    fn get_state(&mut self) -> &mut HtmlState {
        &mut self.state
    }
}

trait HtmlWriter<W: StrWrite> {
    fn get_writer(&mut self) -> &mut W;
    fn get_config(&self) -> &HtmlConfig;
    fn get_state(&mut self) -> &mut HtmlState;
}

#[html_writer(skip_docs)]
#[derive(Debug)]
struct NoDocsWriter<W> {
    base: HtmlWriterBase<W>,
}

fn main() {}
