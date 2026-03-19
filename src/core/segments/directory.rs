use crate::core::segments::Segment;
use crate::model::{InputData, SegmentId};
use crate::utils::shader::shader_by_hex;
use async_trait::async_trait;

pub struct DirectorySegment {
    icon: String,
}
#[async_trait]
impl Segment for DirectorySegment {
    fn create() -> Box<dyn Segment + 'static> {
        Box::new(Self {
            icon: "\u{f024b}".to_string(),
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Directory
    }

    fn get_shader(&self) -> Option<ansi_term::Style> {
        Some(shader_by_hex("#a6da95"))
    }

    fn icon(&self) -> String {
        self.icon.to_string()
    }

    async fn view(&self, config: &InputData) -> Option<String> {
        let path = &config.workspace.current_dir;
        // Handle both Unix and Windows separators by trying both
        let unix_name = path.split('/').next_back().unwrap_or("");
        let windows_name = path.split('\\').next_back().unwrap_or("");

        // Choose the name that indicates actual path splitting occurred
        let result = if windows_name.len() < path.len() {
            // Windows path separator was found
            windows_name
        } else if unix_name.len() < path.len() {
            // Unix path separator was found
            unix_name
        } else {
            // No separator found, use the whole path
            path
        };

        let view = if result.is_empty() {
            "root".to_string()
        } else {
            result.to_string()
        };
        Some(view)
    }
}
