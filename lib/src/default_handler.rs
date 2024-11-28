use crate::config::RendererConfig;
use crate::renderer_state::RendererState;
use crate::tag_handler::TagHandler;

pub struct DefaultTagHandler<'a> {
    pub(crate) state: RendererState,
    pub(crate) config: &'a RendererConfig,
    pub(crate) output: &'a mut String,
}

impl<'a> DefaultTagHandler<'a> {
    pub fn new(output: &'a mut String, config: &'a RendererConfig) -> Self {
        Self {
            state: RendererState::new(),
            config,
            output,
        }
    }
}

impl<'a> TagHandler for DefaultTagHandler<'a> {
    fn get_config(&self) -> &RendererConfig {
        self.config
    }

    fn get_output(&mut self) -> &mut String {
        self.output
    }

    fn get_state(&mut self) -> &mut RendererState {
        &mut self.state
    }
}
