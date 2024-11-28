use crate::html::config::HtmlConfig;
use crate::html::state::HtmlState;
use crate::html::writer::HtmlWriter;

pub struct DefaultHtmlWriter<'a> {
    pub(crate) state: HtmlState,
    pub(crate) config: &'a HtmlConfig,
    pub(crate) output: &'a mut String,
}

impl<'a> DefaultHtmlWriter<'a> {
    pub fn new(output: &'a mut String, config: &'a HtmlConfig) -> Self {
        Self {
            state: HtmlState::new(),
            config,
            output,
        }
    }
}

impl<'a> HtmlWriter for DefaultHtmlWriter<'a> {
    fn get_config(&self) -> &HtmlConfig {
        self.config
    }

    fn get_output(&mut self) -> &mut String {
        self.output
    }

    fn get_state(&mut self) -> &mut HtmlState {
        &mut self.state
    }
}
