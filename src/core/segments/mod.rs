pub mod context_window;
pub mod directory;
pub mod extra;
pub mod git;
pub mod model;
pub mod provider;

use async_trait::async_trait;

use crate::model::{InputData, SegmentId};

#[async_trait]
pub trait Segment: Send + Sync {
    fn new() -> Box<dyn Segment>
    where
        Self: Sized;

    async fn view(&self, config: &InputData) -> Option<String>;
    fn icon(&self) -> String;
    fn get_shader(&self) -> Option<Style>;
    fn id(&self) -> SegmentId;

    async fn render(&self, config: &InputData) -> String {
        if let Some(view) = self.view(config).await {
            let view_text = format!("{} {}", self.icon(), view);
            if let Some(shader) = self.get_shader() {
                return shader.paint(view_text).to_string();
            }
        };
        "".to_string()
    }
}

use ansi_term::Style;
pub use context_window::ContextWindowSegment;
pub use directory::DirectorySegment;
pub use extra::ExtraSegment;
pub use git::GitSegment;
pub use model::ModelSegment;
