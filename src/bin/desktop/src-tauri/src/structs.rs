
#[derive(serde::Serialize)]
pub struct Attemption {
    word: String,
    statuses: [usize; 5]
}

#[derive(serde::Serialize)]
pub struct GameSession {
    target: String,
    attemptions: [Attemption; 6],
    current_attempt: usize,
    completed: bool
}