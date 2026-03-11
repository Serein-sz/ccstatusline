use std::fmt::Write;
use std::process::Command;

use crate::core::segments::Segment;
use crate::model::{InputData, SegmentId};
use crate::utils::shader::shader_by_hex;
use async_trait::async_trait;

#[derive(Debug)]
pub struct GitInfo {
    pub branch: String,
    pub status: GitStatus,
    pub ahead: u32,
    pub behind: u32,
    pub sha: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum GitStatus {
    Clean,
    Dirty,
    Conflicts,
}

pub struct GitSegment {
    icon: String,
}

#[async_trait]
impl Segment for GitSegment {
    fn new() -> Box<dyn Segment + 'static> {
        Box::new(Self {
            icon: "\u{f02a2}".to_string(),
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Model
    }

    fn get_shader(&self) -> Option<ansi_term::Style> {
        Some(shader_by_hex("#91d7e3"))
    }

    fn icon(&self) -> String {
        self.icon.to_string()
    }

    async fn view(&self, config: &InputData) -> Option<String> {
        let working_dir = &config.workspace.project_dir;
        let mut view = String::new();
        if let Some(branch) = self.get_branch(working_dir) {
            write!(&mut view, "{}", branch).unwrap();
            let status = self.get_status(working_dir);
            write!(
                &mut view,
                " {}",
                match status {
                    GitStatus::Clean => "✓",
                    GitStatus::Dirty => "●",
                    GitStatus::Conflicts => "⚠",
                }
            )
            .unwrap();
            return Some(view.to_string());
        }
        None
    }
}

impl GitSegment {
    fn get_branch(&self, working_dir: &String) -> Option<String> {
        if let Ok(output) = Command::new("git")
            .args(["--no-optional-locks", "branch", "--show-current"])
            .current_dir(working_dir)
            .output()
        {
            if output.status.success() {
                let branch = String::from_utf8(output.stdout)
                    .ok()
                    .unwrap()
                    .trim()
                    .to_string();
                if !branch.is_empty() {
                    return Some(branch);
                }
            }
        }

        if let Ok(output) = Command::new("git")
            .args(["--no-optional-locks", "symbolic-ref", "--short", "HEAD"])
            .current_dir(working_dir)
            .output()
        {
            if output.status.success() {
                let branch = String::from_utf8(output.stdout)
                    .ok()
                    .unwrap()
                    .trim()
                    .to_string();
                if !branch.is_empty() {
                    return Some(branch);
                }
            }
        }
        None
    }

    fn get_status(&self, working_dir: &str) -> GitStatus {
        let output = Command::new("git")
            .args(["--no-optional-locks", "status", "--porcelain"])
            .current_dir(working_dir)
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let status_text = String::from_utf8(output.stdout).unwrap_or_default();

                if status_text.trim().is_empty() {
                    return GitStatus::Clean;
                }

                if status_text.contains("UU")
                    || status_text.contains("AA")
                    || status_text.contains("DD")
                {
                    GitStatus::Conflicts
                } else {
                    GitStatus::Dirty
                }
            }
            _ => GitStatus::Clean,
        }
    }
}
