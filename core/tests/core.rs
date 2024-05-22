use core::value_objects as vo;


#[test]
fn test_letter_statuses() {
    let status = vo::LetterStatus::InCorrectPosition;
    assert!(status == vo::LetterStatus::InCorrectPosition);
    assert!(status != vo::LetterStatus::NotFound)
}