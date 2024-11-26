use pulldown_cmark::{Alignment, LinkType};

/// Represents the current state of table parsing
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TableState {
    /// Not currently within a table
    NotInTable,
    /// Currently in the header section of a table
    InHeader,
    /// Currently in the body section of a table
    InBody,
}

impl Default for TableState {
    fn default() -> Self {
        TableState::NotInTable
    }
}

/// Represents the type of list currently being processed
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ListType {
    /// An ordered list (<ol>) with a starting number
    Ordered(u32),
    /// An unordered list (<ul>)
    Unordered,
}

impl Default for ListType {
    fn default() -> Self {
        ListType::Unordered
    }
}

/// Maintains the state of the HTML rendering process
pub struct RendererState {
    /// Stack for tracking list numbers in ordered lists
    pub numbers: Vec<u32>,
    /// Current state of table processing
    pub table_state: TableState,
    /// Current index when processing table cells
    pub table_cell_index: usize,
    /// Alignments for table columns
    pub table_alignments: Vec<Alignment>,
    /// Stack for tracking nested lists
    pub list_stack: Vec<ListType>,
    /// Stack for tracking nested links
    pub link_stack: Vec<LinkType>,
    /// Stack for tracking heading IDs
    pub heading_stack: Vec<String>,
    /// Whether currently processing a code block
    pub currently_in_code_block: bool,
}

impl RendererState {
    /// Create a new renderer state with default values
    pub fn new() -> Self {
        Self {
            numbers: Vec::new(),
            table_state: TableState::default(),
            table_cell_index: 0,
            table_alignments: Vec::new(),
            list_stack: Vec::new(),
            link_stack: Vec::new(),
            heading_stack: Vec::new(),
            currently_in_code_block: false,
        }
    }

    /// Reset all state, typically called between document renders
    pub fn reset(&mut self) {
        self.numbers.clear();
        self.table_state = TableState::default();
        self.table_cell_index = 0;
        self.table_alignments.clear();
        self.list_stack.clear();
        self.link_stack.clear();
        self.heading_stack.clear();
        self.currently_in_code_block = false;
    }

    /// Check if currently inside a table
    pub fn in_table(&self) -> bool {
        self.table_state != TableState::NotInTable
    }

    /// Check if currently in a table header
    pub fn in_table_header(&self) -> bool {
        self.table_state == TableState::InHeader
    }

    /// Get the current nesting level of lists
    pub fn list_depth(&self) -> usize {
        self.list_stack.len()
    }

    /// Get the current list type, if any
    pub fn current_list_type(&self) -> Option<ListType> {
        self.list_stack.last().copied()
    }
}

impl Default for RendererState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_state_new() {
        let state = RendererState::new();
        assert_eq!(state.table_state, TableState::NotInTable);
        assert_eq!(state.table_cell_index, 0);
        assert!(state.numbers.is_empty());
        assert!(state.table_alignments.is_empty());
        assert!(state.list_stack.is_empty());
        assert!(state.link_stack.is_empty());
        assert!(state.heading_stack.is_empty());
        assert!(!state.currently_in_code_block);
    }

    #[test]
    fn test_renderer_state_reset() {
        let mut state = RendererState::new();

        // Modify state
        state.numbers.push(1);
        state.table_state = TableState::InHeader;
        state.table_cell_index = 2;
        state.list_stack.push(ListType::Ordered(1));
        state.currently_in_code_block = true;

        // Reset
        state.reset();

        // Verify reset
        assert_eq!(state.table_state, TableState::NotInTable);
        assert_eq!(state.table_cell_index, 0);
        assert!(state.numbers.is_empty());
        assert!(state.list_stack.is_empty());
        assert!(!state.currently_in_code_block);
    }

    #[test]
    fn test_list_operations() {
        let mut state = RendererState::new();

        assert_eq!(state.list_depth(), 0);
        assert_eq!(state.current_list_type(), None);

        state.list_stack.push(ListType::Unordered);
        assert_eq!(state.list_depth(), 1);
        assert_eq!(state.current_list_type(), Some(ListType::Unordered));

        state.list_stack.push(ListType::Ordered(1));
        assert_eq!(state.list_depth(), 2);
        assert_eq!(state.current_list_type(), Some(ListType::Ordered(1)));
    }

    #[test]
    fn test_table_state() {
        let mut state = RendererState::new();

        assert!(!state.in_table());
        assert!(!state.in_table_header());

        state.table_state = TableState::InHeader;
        assert!(state.in_table());
        assert!(state.in_table_header());

        state.table_state = TableState::InBody;
        assert!(state.in_table());
        assert!(!state.in_table_header());
    }
}
