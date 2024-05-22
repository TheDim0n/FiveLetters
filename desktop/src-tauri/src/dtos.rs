use fiveletters::value_objects as vo;

#[derive(serde::Serialize)]
pub struct Attemption {
    pub word: String,
    pub statuses: [u8; vo::WORD_SIZE]
}

#[derive(serde::Serialize)]
pub struct GameSession {
    pub id: usize,
    pub target: String,
    pub attemptions: [Attemption; vo::ATTEMPT_COUNT],
    pub current_attempt: usize,
    pub completed: bool
}