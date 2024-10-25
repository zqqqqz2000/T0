#[derive(Debug)]
pub enum Filter {
    SqlFilter(Option<String>),
    RegexFilter(Option<String>),
}

#[derive(Debug)]
pub enum Action {
    AddTag(Option<String>),
    DeleteTag(Option<String>),
    CursorUp,
    CursorDown,
    SelectAll,
    UnselectAll,
    Reindex,
}

#[derive(Debug)]
pub enum Event {
    Filter(Filter),
    Action(Action),
}

#[derive(Debug)]
pub struct Keymap {
    pub key: String,
    pub event: Event,
}

#[derive(Debug)]
pub struct TagsStyle {
    pub text_color: String,
    pub bg_color: String,
    pub name: String,
}

#[derive(Debug)]
pub enum Condition {
    Normal,
    Completed,
    Timeout,
}

#[derive(Debug)]
pub struct ItemStyle {
    pub text_color: String,
    pub bg_color: String,
    pub condition: Condition,
}

#[derive(Debug)]
pub struct Config {
    pub keymaps: Vec<Keymap>,
    pub default_filters: Vec<Filter>,
    pub tags_style: Vec<TagsStyle>,
    pub default_tag_style: TagsStyle,
    pub default_tag: String,
    pub date_input_schema: String,
    pub date_display_schema: String,
    pub item_style: Vec<ItemStyle>,
}
