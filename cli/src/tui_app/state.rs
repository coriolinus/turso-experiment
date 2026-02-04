use std::collections::HashMap;

use todo_list::TodoListId;

/// Application state
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum State {
    #[default]
    Initial,
    ListSelect(HashMap<TodoListId, String>),
    Exit,
}
