use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Filter {
    SqlFilter(Option<String>),
    RegexFilter(Option<String>),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Action {
    AddTag(Option<String>),
    DeleteTag(Option<String>),
    CursorUp,
    CursorDown,
    SelectAll,
    UnselectAll,
    Reindex,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Event {
    Filter(Filter),
    Action(Action),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Keymap {
    pub key: String,
    pub event: Event,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagsStyle {
    pub text_color: String,
    pub bg_color: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Condition {
    Normal,
    Completed,
    Timeout,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemStyle {
    pub text_color: String,
    pub bg_color: String,
    pub condition: Condition,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EncryptMethod {
    DES { cipher_path: String },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ChangeCallback {
    SaveToGit { repo: String },
    Execute { command: String },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub todo_file: String,
    pub encrypt_method: Option<EncryptMethod>,
    pub change_callback: Option<ChangeCallback>,
    pub keymaps: Vec<Keymap>,
    pub default_filters: Vec<Filter>,
    pub tags_style: Vec<TagsStyle>,
    pub default_tag_style: TagsStyle,
    pub default_tag: Option<String>,
    pub date_input_format: String,
    pub date_display_format: String,
    pub item_style: Vec<ItemStyle>,
}
