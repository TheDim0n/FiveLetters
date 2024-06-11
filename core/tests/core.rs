use core::{
    entities,
    value_objects as vo
};


#[test]
fn test_letter_statuses() {
    let status = vo::LetterStatus::InCorrectPosition;
    assert!(status == vo::LetterStatus::InCorrectPosition);
    assert!(status != vo::LetterStatus::NotFound)
}

#[test]
fn test_add_attentions() {
    let mut game_session = entities::GameSession::from((1, "абвгд"));
    assert!(game_session.current_attempt == 1);

    game_session.add_attemption(&"ттттт".to_owned(), 1);
    assert!(game_session.current_attempt == 2);
    let statuses = game_session.attemptions[0].statuses;
    assert!(statuses == [vo::LetterStatus::NotFound; vo::WORD_SIZE]);

    game_session.add_attemption(&"тбабх".to_owned(), 2);
    assert!(game_session.current_attempt == 3);
    let statuses = game_session.attemptions[1].statuses;
    assert!(statuses == [
        vo::LetterStatus::NotFound,
        vo::LetterStatus::InCorrectPosition,
        vo::LetterStatus::InUncorrectPosition,
        vo::LetterStatus::NotFound,
        vo::LetterStatus::NotFound,
    ], "{:#?}", statuses);

    game_session.add_attemption(&"тгагж".to_owned(), 3);
    assert!(game_session.current_attempt == 4);
    let statuses = game_session.attemptions[2].statuses;
    assert!(statuses == [
        vo::LetterStatus::NotFound,
        vo::LetterStatus::NotFound,
        vo::LetterStatus::InUncorrectPosition,
        vo::LetterStatus::InCorrectPosition,
        vo::LetterStatus::NotFound,
    ], "{:#?}", statuses);

    game_session.add_attemption(&"бтхбн".to_owned(), 4);
    assert!(game_session.current_attempt == 5);
    let statuses = game_session.attemptions[3].statuses;
    assert!(statuses == [
        vo::LetterStatus::NotFound,
        vo::LetterStatus::NotFound,
        vo::LetterStatus::NotFound,
        vo::LetterStatus::InUncorrectPosition,
        vo::LetterStatus::NotFound,
    ], "{:#?}", statuses);
}

#[test]
fn test_add_attentions_more() {
    let mut game_session = entities::GameSession::from((1, "слава"));

    game_session.add_attemption(&"факел".to_owned(), 1);
    let statuses = game_session.attemptions[0].statuses;
    assert!(statuses == [
        vo::LetterStatus::NotFound,
        vo::LetterStatus::InUncorrectPosition,
        vo::LetterStatus::NotFound,
        vo::LetterStatus::NotFound,
        vo::LetterStatus::InUncorrectPosition,
    ], "{:#?}", statuses);

    game_session.add_attemption(&"оклад".to_owned(), 2);
    let statuses = game_session.attemptions[1].statuses;
    assert!(statuses == [
        vo::LetterStatus::NotFound,
        vo::LetterStatus::NotFound,
        vo::LetterStatus::InUncorrectPosition,
        vo::LetterStatus::InUncorrectPosition,
        vo::LetterStatus::NotFound,
    ], "{:#?}", statuses);

    game_session.add_attemption(&"абзац".to_owned(), 3);
    let statuses = game_session.attemptions[2].statuses;
    assert!(statuses == [
        vo::LetterStatus::InUncorrectPosition,
        vo::LetterStatus::NotFound,
        vo::LetterStatus::NotFound,
        vo::LetterStatus::InUncorrectPosition,
        vo::LetterStatus::NotFound,
    ], "{:#?}", statuses);
}
