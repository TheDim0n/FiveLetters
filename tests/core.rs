use fiveletters::core::entities;
use fiveletters::core::value_objects as vo;


#[test]
fn test_letter_statuses() {
    let status = vo::LetterStatus::InCorrectPosition;
    assert!(status == vo::LetterStatus::InCorrectPosition);
    assert!(status != vo::LetterStatus::NotFound)
}

#[test]
fn test_init_attempt() {
    let attempt = entities::Attempt::new("Слово");
    assert!(attempt.current() == &0);
    assert_eq!(attempt.statuses().len(), 5);
    assert_eq!(attempt.solution(), &String::from("слово"));
}

#[test]
fn test_check_attempt() {
    let mut attempt = entities::Attempt::new("слово");
    let status = attempt.check("олово").unwrap();
    assert!(!status);
    assert_eq!(attempt.current(), &1);

    let status = attempt.check("Слово").unwrap();
    assert_eq!(attempt.statuses(), &vec![vo::LetterStatus::InCorrectPosition; 5]);
    assert!(status);
    assert_eq!(attempt.current(), &2);
}