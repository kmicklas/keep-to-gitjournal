use serde::Deserialize;
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub is_trashed: bool,
    pub is_archived: bool,
    pub text_content: String,
    pub title: String,
    pub user_edited_timestamp_usec: i64,
    pub created_timestamp_usec: i64,
    pub labels: Vec<Label>,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub name: String,
}

pub fn read_notes(dir: &Path) -> anyhow::Result<HashMap<PathBuf, Note>> {
    let mut notes = HashMap::new();

    for child in dir.read_dir()? {
        let child = child?;
        let path = child.path();
        if child.file_type()?.is_file() && path.extension() == Some(OsStr::new("json")) {
            let note = serde_json::from_reader(File::open(&path)?)?;
            notes.insert(path, note);
        }
    }
    Ok(notes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let data = r#"{"color":"DEFAULT","isTrashed":false,"isPinned":false,"isArchived":false,"textContent":"content","title":"title","userEditedTimestampUsec":1441394812887000,"createdTimestampUsec":1412018652099000,"labels":[{"name":"Reference"}]}"#;

        let note: Note = serde_json::from_str(data).unwrap();

        assert_eq!(
            note,
            Note {
                is_trashed: false,
                is_archived: false,
                text_content: "content".to_owned(),
                title: "title".to_owned(),
                user_edited_timestamp_usec: 1441394812887000,
                created_timestamp_usec: 1412018652099000,
                labels: vec![Label {
                    name: "Reference".to_owned()
                }]
            }
        );
    }
}
