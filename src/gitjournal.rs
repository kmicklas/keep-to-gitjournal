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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_markdown() {
        let mut markdown = Vec::new();

        Note {
            is_trashed: false,
            is_archived: false,
            text_content: "content".to_owned(),
            title: "title".to_owned(),
            user_edited_timestamp_usec: 1441394812887000,
            created_timestamp_usec: 1412018652099000,
        }
        .write_markdown(&mut markdown)
        .unwrap();

        assert_eq!(
            String::from_utf8_lossy(&markdown),
            r#"---
---

# title

content
"#
        );
    }
}
