use fiveletters::core::entities;
use fiveletters::core::value_objects as vo;


#[test]
fn test_letter_statuses() {
    let status = vo::LetterStatus::InCorrectPosition;
    assert!(status == vo::LetterStatus::InCorrectPosition);
    assert!(status != vo::LetterStatus::NotFound)
}

#[test]
fn test_init_game_session() {
    let attempt = entities::GameSession::from("Слово");
    assert!(attempt.current_attempt() == &0);
    assert_eq!(attempt.statuses().len(), vo::ATTEMPT_COUNT);
    assert_eq!(attempt.solution(), &String::from("слово"));
}

#[test]
fn test_check_attempt() {
    let mut attempt = entities::GameSession::from("слово");
    let status = attempt.check("олово").unwrap();
    assert!(!status);
    assert_eq!(attempt.current_attempt(), &1);

    let status = attempt.check("Слово").unwrap();
    assert_eq!(attempt.statuses(), &[
        [
            vo::LetterStatus::InUncorrectPosition,
            vo::LetterStatus::InCorrectPosition,
            vo::LetterStatus::InCorrectPosition,
            vo::LetterStatus::InCorrectPosition,
            vo::LetterStatus::InCorrectPosition
        ],
        [vo::LetterStatus::InCorrectPosition; vo::WORD_SIZE],
        [vo::LetterStatus::Undefined; vo::WORD_SIZE],
        [vo::LetterStatus::Undefined; vo::WORD_SIZE],
        [vo::LetterStatus::Undefined; vo::WORD_SIZE],
        [vo::LetterStatus::Undefined; vo::WORD_SIZE]
    ]);
    assert!(status);
    assert_eq!(attempt.current_attempt(), &2);
}