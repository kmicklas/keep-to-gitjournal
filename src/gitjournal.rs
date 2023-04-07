use std::path::{Path, PathBuf};

use crate::keep::Note;

pub fn file_name(path: &Path) -> PathBuf {
    path.with_extension("md")
        .file_name()
        .expect("expected file name")
        .into()
}

fn to_datetime(usec: i64) -> String {
    chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp_micros(usec).expect("timestamp must be valid"),
        chrono::Utc,
    )
    .to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
}

impl Note {
    pub fn write_markdown(&self, out: &mut impl std::io::Write) -> anyhow::Result<()> {
        writeln!(out, "---")?;
        writeln!(out, "created: {}", to_datetime(self.created_timestamp_usec))?;
        writeln!(
            out,
            "modified: {}",
            to_datetime(self.user_edited_timestamp_usec)
        )?;
        if !self.labels.is_empty() {
            let names: Vec<&str> = self.labels.iter().map(|l| l.name.as_str()).collect();
            writeln!(out, "tags: [{}]", names.join(", "))?;
        }
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
    use crate::keep::Label;

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
            labels: vec![
                Label {
                    name: "Reference".to_owned(),
                },
                Label {
                    name: "Other".to_owned(),
                },
            ],
        }
        .write_markdown(&mut markdown)
        .unwrap();

        assert_eq!(
            String::from_utf8_lossy(&markdown),
            r#"---
created: 2014-09-29T19:24:12+00:00
modified: 2015-09-04T19:26:52+00:00
tags: [Reference, Other]
---

# title

content
"#
        );
    }
}
