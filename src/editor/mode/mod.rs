// Ante is a modal text editor.
pub enum Mode {
    /* To navigate across the current text buffer
    and across the editor itself (tabs, windows, menus, text_buffer collection ). */
    Navigation,
    /* To edit the targeted text buffer. */
    Edition,
    /* To copy, cut, move a blob of text into the targeted text buffer.
    Ante will work with an internal clipboard buffer. */
    Selection,
}

impl Mode {
    pub fn new() -> Self {
        Self::Navigation
    }
    pub fn switch_to_navigation(&mut self) {
        *self = Self::Navigation;
    }
    pub fn switch_to_edition(&mut self) {
        *self = Self::Edition;
    }
    pub fn switch_to_selection(&mut self) {
        *self = Self::Selection;
    }
    pub fn is_navigation_mode(&self) -> bool {
        match *self {
            Self::Navigation => true,
            Self::Edition => false,
            Self::Selection => false,
        }  
    }
    pub fn is_edition_mode(&self) -> bool {
        match *self {
            Self::Navigation => false,
            Self::Edition => true,
            Self::Selection => false,
        }  
    }
    pub fn is_selection_mode(&self) -> bool {
        match *self {
            Self::Navigation => false,
            Self::Edition => false,
            Self::Selection => true,
        }  
    }
}