use super::config::RendererConfig;
use super::renderer_state::RendererState;
use super::tag_handler::TagHandler;
use pulldown_cmark::{Alignment, CodeBlockKind, HeadingLevel, LinkType};

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
    fn write_str(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn write_attributes(&mut self, element: &str) {
        if let Some(attrs) = self.config.attributes.element_attributes.get(element) {
            for (key, value) in attrs {
                self.write_str(&format!(" {}=\"{}\"", key, value));
            }
        }
    }

    fn is_external_link(&self, url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }

    fn heading_level_to_u8(&self, level: HeadingLevel) -> u8 {
        match level {
            HeadingLevel::H1 => 1,
            HeadingLevel::H2 => 2,
            HeadingLevel::H3 => 3,
            HeadingLevel::H4 => 4,
            HeadingLevel::H5 => 5,
            HeadingLevel::H6 => 6,
        }
    }

    fn generate_heading_id(&self, level: HeadingLevel) -> String {
        let level_num = self.heading_level_to_u8(level);
        format!("{}{}", self.config.elements.headings.id_prefix, level_num)
    }

    fn start_heading(&mut self, level: HeadingLevel, id: Option<&str>, classes: Vec<&str>) {
        let tag = format!("h{}", self.heading_level_to_u8(level));
        self.write_str(&format!("<{}", tag));

        if self.config.elements.headings.add_ids {
            let heading_id = match id {
                Some(id) => id.to_string(),
                None => self.generate_heading_id(level),
            };
            self.write_str(&format!(" id=\"{}\"", heading_id));
            self.state.heading_stack.push(heading_id);
        }

        let mut all_classes = Vec::new();
        let level_num = self.heading_level_to_u8(level);
        if let Some(level_class) = self.config.elements.headings.level_classes.get(&level_num) {
            all_classes.push(level_class.clone());
        }
        all_classes.extend(classes.into_iter().map(|s| s.to_string()));

        if !all_classes.is_empty() {
            self.write_str(" class=\"");
            self.write_str(&all_classes.join(" "));
            self.write_str("\"");
        }

        self.write_attributes(&tag);
        self.write_str(">");
    }

    fn start_code_block(&mut self, kind: CodeBlockKind) {
        self.state.currently_in_code_block = true;
        self.write_str("<pre");
        self.write_attributes("pre");
        self.write_str("><code");

        match kind {
            CodeBlockKind::Fenced(info) => {
                let lang = if info.is_empty() {
                    self.config.elements.code_blocks.default_language.as_deref()
                } else {
                    Some(&*info)
                };

                if let Some(lang) = lang {
                    self.write_str(&format!(" class=\"language-{}\"", lang));
                }
            }
            CodeBlockKind::Indented => {
                if let Some(lang) = &self.config.elements.code_blocks.default_language {
                    self.write_str(&format!(" class=\"language-{}\"", lang));
                }
            }
        }

        self.write_attributes("code");
        self.write_str(">");
    }

    fn start_list(&mut self, first_number: Option<u64>) {
        match first_number {
            Some(n) => {
                self.state.numbers.push(n.try_into().unwrap());
                self.state
                    .list_stack
                    .push(super::renderer_state::ListType::Ordered(
                        n.try_into().unwrap(),
                    ));
                self.write_str("<ol");
                if n != 1 {
                    self.write_str(&format!(" start=\"{}\"", n));
                }
                self.write_attributes("ol");
                self.write_str(">");
            }
            None => {
                self.state
                    .list_stack
                    .push(super::renderer_state::ListType::Unordered);
                self.write_str("<ul");
                self.write_attributes("ul");
                self.write_str(">");
            }
        }
    }

    fn start_list_item(&mut self) {
        self.write_str("<li");
        self.write_attributes("li");
        self.write_str(">");
    }

    fn start_table(&mut self, alignments: Vec<Alignment>) {
        self.state.table_state = super::renderer_state::TableState::InHeader;
        self.state.table_alignments = alignments;
        self.write_str("<table");
        self.write_attributes("table");
        self.write_str(">");
    }

    fn start_table_head(&mut self) {
        self.state.table_state = super::renderer_state::TableState::InHeader;
        self.state.table_cell_index = 0;
        self.write_str("<thead><tr>");
    }

    fn start_table_row(&mut self) {
        self.state.table_cell_index = 0;
        self.write_str("<tr>");
    }

    fn start_table_cell(&mut self) {
        let tag = match self.state.table_state {
            super::renderer_state::TableState::InHeader => "th",
            _ => "td",
        };

        self.write_str("<");
        self.write_str(tag);

        if let Some(alignment) = self.state.table_alignments.get(self.state.table_cell_index) {
            match alignment {
                Alignment::Left => self.write_str(" style=\"text-align: left\""),
                Alignment::Center => self.write_str(" style=\"text-align: center\""),
                Alignment::Right => self.write_str(" style=\"text-align: right\""),
                Alignment::None => {}
            }
        }

        self.write_attributes(tag);
        self.write_str(">");

        self.state.table_cell_index += 1;
    }

    fn start_link(&mut self, link_type: LinkType, dest: &str, title: &str) {
        self.state.link_stack.push(link_type);
        self.write_str("<a href=\"");
        // Use the escape_href function from utils
        super::utils::escape_href(self.output, dest);
        self.write_str("\"");

        if !title.is_empty() {
            self.write_str(" title=\"");
            super::utils::escape_html(self.output, title);
            self.write_str("\"");
        }

        if self.is_external_link(dest) {
            if self.config.elements.links.nofollow_external {
                self.write_str(" rel=\"nofollow\"");
            }
            if self.config.elements.links.open_external_blank {
                self.write_str(" target=\"_blank\"");
            }
        }

        self.write_attributes("a");
        self.write_str(">");
    }

    fn start_image(&mut self, _link_type: LinkType, dest: &str, title: &str) {
        self.write_str("<img src=\"");
        super::utils::escape_href(self.output, dest);
        self.write_str("\" alt=\"");

        if !title.is_empty() {
            self.write_str(" title=\"");
            super::utils::escape_html(self.output, title);
            self.write_str("\"");
        }

        self.write_attributes("img");

        if self.config.html.xhtml_style {
            self.write_str(" />");
        } else {
            self.write_str(">");
        }
    }
}
