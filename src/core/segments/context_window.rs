use crate::core::segments::Segment;
use crate::model::{InputData, SegmentId};
use crate::utils::shader::shader_by_hex;
use async_trait::async_trait;

pub struct ContextWindowSegment {
    icon: String,
}

#[async_trait]
impl Segment for ContextWindowSegment {
    fn new() -> Box<dyn Segment + 'static> {
        Box::new(Self {
            icon: "\u{f1c0}".to_string(),
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::ContextWindow
    }

    fn get_shader(&self) -> Option<ansi_term::Style> {
        Some(shader_by_hex("#ee99a0"))
    }

    fn icon(&self) -> String {
        self.icon.to_string()
    }

    async fn view(&self, config: &InputData) -> Option<String> {
        let context_window = config.context_window.as_ref().unwrap();
        let used_percentage = context_window.used_percentage.unwrap_or_default();
        let context_window_size =
            (context_window.context_window_size.unwrap_or_default() * used_percentage / 100) / 1000;
        Some(format!("{}% · {}k tokens", used_percentage, context_window_size))
    }
}
