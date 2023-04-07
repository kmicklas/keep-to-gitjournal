use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    is_trashed: bool,
    is_archived: bool,
    text_content: String,
    title: String,
    user_edited_timestamp_usec: u64,
    created_timestamp_usec: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let data = r#"{"color":"DEFAULT","isTrashed":false,"isPinned":false,"isArchived":false,"textContent":"( ͡° ͜ʖ ͡°)\n( ͠° ͟ʖ ͡°)\n(ง\u2019̀-\u2019́)ง","title":"Emojis","userEditedTimestampUsec":1441394812887000,"createdTimestampUsec":1412018652099000,"labels":[{"name":"Reference"}]}"#;

        let note: Note = serde_json::from_str(data).unwrap();

        assert_eq!(
            note,
            Note {
                is_trashed: false,
                is_archived: false,
                text_content: "( \u{361}° \u{35c}ʖ \u{361}°)\n( \u{360}° \u{35f}ʖ \u{361}°)\n(ง’\u{300}-’\u{301})ง".to_owned(),
                title: "Emojis".to_owned(),
                user_edited_timestamp_usec: 1441394812887000,
                created_timestamp_usec: 1412018652099000,
            }
        );
    }
}
