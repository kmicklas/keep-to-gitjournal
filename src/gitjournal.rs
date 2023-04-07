use std::path::{Path, PathBuf};

use crate::keep::Note;

pub fn file_name(path: &Path) -> PathBuf {
    path.with_extension("md")
        .file_name()
        .expect("expected file name")
        .into()
}

impl Note {
    pub fn write_markdown(&self, out: &mut impl std::io::Write) -> anyhow::Result<()> {
        writeln!(out, "---")?;
        writeln!(out, "---")?;
        writeln!(out)?;
        writeln!(out, "# {}", self.title)?;
        writeln!(out)?;
        writeln!(out, "{}", self.text_content)?;
        Ok(())
    }
}
