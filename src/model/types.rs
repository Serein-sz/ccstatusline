use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SegmentId {
    Model,
    Directory,
    Git,
    ContextWindow,
    Extra
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InputData {
    pub model: Model,
    pub workspace: Workspace,
    pub context_window: Option<ContextWindow>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Model {
    pub id: String,
    pub display_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ContextWindow {
    pub context_window_size: Option<i32>,
    pub used_percentage: Option<i32>,
    pub remaining_percentage: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Workspace {
    pub current_dir: String,
    pub project_dir: String,
}
