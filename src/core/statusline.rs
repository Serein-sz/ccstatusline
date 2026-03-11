use crate::core::segments::{
    ContextWindowSegment, DirectorySegment, ExtraSegment, GitSegment, ModelSegment,
};
use crate::{core::segments::Segment, model::InputData};
pub struct StatusLine {}

impl StatusLine {
    pub async fn generate(config: &InputData) -> String {
        let active_segments: Vec<Box<dyn Segment>> = vec![
            ModelSegment::new(),
            DirectorySegment::new(),
            GitSegment::new(),
            ContextWindowSegment::new(),
            ExtraSegment::new(),
        ];
        // 使用 futures::future::join_all 并发执行
        let segments: Vec<String> =
            futures::future::join_all(active_segments.iter().map(|s| s.render(config))).await;

        segments
            .into_iter()
            .filter(|segment| !segment.is_empty())
            .collect::<Vec<_>>()
            .join(" | ")
    }
}
