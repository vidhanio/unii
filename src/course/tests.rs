use super::*;

#[test]
fn test_code_parsing() {
    let course_codes = ["1MD3", "", "ABC"];
    let expected = [
        Ok(Code {
            year: Year::First,
            rest: "MD3".to_string(),
        }),
        Err(CodeError::InvalidLength),
        Err(CodeError::InvalidYear),
    ];

    for (code, expected) in course_codes.iter().zip(expected) {
        assert_eq!(code.parse::<Code>(), expected);
    }
}
