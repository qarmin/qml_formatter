#[derive(Eq, PartialEq, Debug)]
pub enum UserTextOrNot {
    UserText,
    QMLCode,
}

pub fn split_text_into_comment_part(text: &str, mut current_separator: Option<char>) -> Vec<(UserTextOrNot, String)> {
    let mut current_part: Vec<char> = Vec::new();
    let mut parts: Vec<(UserTextOrNot, String)> = Vec::new();
    for i in text.chars() {
        // dbg!(&current_part, &parts);
        if current_separator == None {
            current_part.push(i);
            if ['\'', '"', '`'].contains(&i) {
                current_separator = Some(i);
                parts.push((UserTextOrNot::QMLCode, current_part.into_iter().collect::<String>()));
                current_part = Vec::new();
            }
        } else {
            if current_separator == Some(i) {
                current_separator = None;
                parts.push((UserTextOrNot::UserText, current_part.into_iter().collect::<String>()));
                current_part = Vec::new();
            }
            current_part.push(i);
        }
    }
    if !current_part.is_empty() {
        if current_separator.is_none() {
            parts.push((UserTextOrNot::QMLCode, current_part.into_iter().collect::<String>()));
        } else {
            parts.push((UserTextOrNot::UserText, current_part.into_iter().collect::<String>()));
        }
    }
    parts
}

#[test]
pub fn test_split_into_comment_part() {
    let input = r##"input_text"##;
    let output = vec![(UserTextOrNot::QMLCode, "input_text".to_string())];
    assert_eq!(output, split_text_into_comment_part(input, None));

    let input = r##"input_text:"abcd""##;
    let output = vec![
        (UserTextOrNot::QMLCode, "input_text:\"".to_string()),
        (UserTextOrNot::UserText, "abcd".to_string()),
        (UserTextOrNot::QMLCode, "\"".to_string()),
    ];
    assert_eq!(output, split_text_into_comment_part(input, None));
}
