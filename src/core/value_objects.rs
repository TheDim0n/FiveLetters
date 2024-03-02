#[derive(PartialEq, Clone, Copy, Debug)]

pub enum LetterStatus{Undefined, InUncorrectPosition, InCorrectPosition, NotFound}

pub const ATTEMPT_COUNT: usize = 6;
pub const WORD_SIZE: usize = 5;