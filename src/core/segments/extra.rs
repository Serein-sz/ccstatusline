use crate::core::segments::Segment;
use crate::core::segments::provider::fetch_usage;
use crate::model::{InputData, SegmentId};
use crate::utils::shader::shader_by_hex;
use async_trait::async_trait;

pub struct ExtraSegment {
    icon: String,
}

#[async_trait]
impl Segment for ExtraSegment {
    fn new() -> Box<dyn Segment + 'static> {
        Box::new(Self {
            icon: "\u{f135}".to_string(),
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Extra
    }

    fn get_shader(&self) -> Option<ansi_term::Style> {
        Some(shader_by_hex("#b7bdf8"))
    }

    fn icon(&self) -> String {
        self.icon.to_string()
    }

    async fn view(&self, config: &InputData) -> Option<String> {
        match config.model.display_name.as_str() {
            "MiniMax-M2.5" => Some(fetch_usage().await),
            _ => None,
        }
    }
}
