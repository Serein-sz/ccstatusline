use crate::core::segments::Segment;
use crate::model::{InputData, SegmentId};
use crate::utils::shader::shader_by_hex;
use async_trait::async_trait;

pub struct ModelSegment {
    icon: String,
}

#[async_trait]
impl Segment for ModelSegment {
    fn new() -> Box<dyn Segment + 'static> {
        Box::new(Self {
            icon: "\u{e26d}".to_string(),
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Model
    }

    fn get_shader(&self) -> Option<ansi_term::Style> {
        Some(shader_by_hex("#c6a0f6"))
    }

    fn icon(&self) -> String {
        self.icon.to_string()
    }

    async fn view(&self, config: &InputData) -> String {
        config.model.display_name.to_string()
    }
}
