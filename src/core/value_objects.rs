use std;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum LetterStatus{Undefined, InUncorrectPosition, InCorrectPosition, NotFound}

#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(i8)]
pub enum WordStatuses{Inactive = -1, Active = 0, Solved = 1}
impl std::fmt::Display for WordStatuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as i8)
    }
}


pub const ATTEMPT_COUNT: usize = 6;
pub const WORD_SIZE: usize = 5;