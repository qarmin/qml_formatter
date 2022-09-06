#[cfg(test)]
mod tests {
    use crate::convert_file;
    use crate::rules::*;
    use std::fs;

    #[test]
    fn test_space_before_bracket() {
        let input = r#"Text{"#;
        let expected_output = r#"Text {"#;
        assert_eq!(space_before_bracket(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }
    #[test]
    fn test_if_movement() {
        let input = r#"
    if(abc)
    bcd
"#;
        let expected_output = r#"
    if (abc)
        bcd
"#;
        assert_eq!(if_movement(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }
    #[test]
    fn test_remove_empty_line_before_close_bracket() {
        let input = r#"Text {
text: "TODO"

}"#;
        let expected_output = r#"Text {
text: "TODO"
}"#;
        assert_eq!(remove_empty_line_before_close_bracket(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_connect_multiple_empty_lines_into_one() {
        let input = r#"
Text {}






Text {}
"#;
        let expected_output = r#"
Text {}

Text {}
"#;
        assert_eq!(connect_multiple_empty_lines_into_one(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_move_single_open_bracket() {
        let input = r#"Text
{
}"#;
        let expected_output = r#"Text {
}"#;
        assert_eq!(move_single_open_bracket(split_text_to_vector(input)), split_text_to_vector(expected_output));
        let input = r#"Text

{
}"#;
        let expected_output = r#"Text {
}"#;
        assert_eq!(move_single_open_bracket(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_remove_useless_spaces_around_colon() {
        let input = r#"property var roman   :    ABCD"#;
        let expected_output = r#"property var roman: ABCD"#;
        assert_eq!(remove_useless_spaces_around_colon(split_text_to_vector(input)), split_text_to_vector(expected_output));

        let input = r#"property var roman   :    "ABCD    :    ABCD""#;
        let expected_output = r#"property var roman: "ABCD    :    ABCD""#;
        assert_eq!(remove_useless_spaces_around_colon(split_text_to_vector(input)), split_text_to_vector(expected_output));

        let input = r#"text: "ABCD \" \" \", \":   \" \" \" \" \" \"""#;
        let expected_output = r#"text: "ABCD \" \" \", \":   \" \" \" \" \" \"""#;
        assert_eq!(remove_useless_spaces_around_colon(split_text_to_vector(input)), split_text_to_vector(expected_output));

        let input = r#"property var roman:ABCD"#;
        let expected_output = r#"property var roman:ABCD"#;
        assert_eq!(remove_useless_spaces_around_colon(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_move_elements_inside() {
        let input = r#"
Text {
Label {
Text {}
}
}
"#;
        let expected_output = r#"
Text {
    Label {
        Text {}
    }
}
"#;
        assert_eq!(move_elements_inside(split_text_to_vector(input)), split_text_to_vector(expected_output));
        let input = r#"
Service(
Ser(
Serr()
)
)
"#;
        let expected_output = r#"
Service(
    Ser(
        Serr()
    )
)
"#;
        assert_eq!(move_elements_inside(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_remove_empty_space_on_end_of_line() {
        let input = r#"
        
Text {}         
                 
"#;
        let expected_output = r#"

Text {}

"#;
        assert_eq!(remove_empty_space_on_end_of_line(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_skip_start_end_empty_lines() {
        let input = r#"

Text {}

"#;
        let expected_output = r#"Text {}"#;
        assert_eq!(skip_start_end_empty_lines(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_conversion() {
        let test_dir = "src/test_data/";
        let tests = [
            "1_empty_space_at_the_end",
            "2_empty_lines_at_start_end",
            "3_mismatched_space",
            "4_not_mismatched_space",
            "5_not_mismatched_space_inside",
            "6_space_after_colon",
            "7_space_after_colon_in_string",
            "8_space_after_colon_in_string_with_slash",
            "9_single_open_bracket",
            "10_multiple_empty_lines",
            "11_empty_line_before_ending",
            "12_if_else_mismatch",
            "13_if_oneliner",
            "14_different_quotes",
            "15_square_bracket",
            "16_space_after_colon_ternary",
            "17_no_space_when_between_brackets",
            "example",
        ];
        for test_name in tests {
            println!("Testing {}", test_name);
            let input_name = format!("{}{}_INPUT.qml", test_dir, test_name);
            let output_name = format!("{}{}_OUTPUT.qml", test_dir, test_name);

            let input_content = fs::read_to_string(input_name).unwrap();
            let output_content = fs::read_to_string(output_name).unwrap();

            assert_eq!(convert_file(input_content), output_content);
        }
    }
}
