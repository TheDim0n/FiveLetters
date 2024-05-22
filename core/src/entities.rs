use crate::value_objects as vo;


type Attemptions = [vo::Attemption; vo::ATTEMPT_COUNT];


#[derive(Debug)]
pub struct GameSession {
    pub id: usize,
    pub target: String,
    pub attemptions: Attemptions,
    pub current_attempt: usize,
    pub completed: bool
}

impl From<(usize, &str)> for GameSession {
    fn from((id, world): (usize, &str)) -> GameSession {
        let attemptions: Attemptions = std::array::from_fn(|_| vo::Attemption::empty());
        GameSession{
            id,
            target: String::from(world.to_lowercase()),
            attemptions,
            current_attempt: 0,
            completed: false
        }
    }
}

impl GameSession {
    pub fn add_attemption(&mut self, value: &String, mut number: usize) {
        if vo::WORD_SIZE != value.chars().count() {
            panic!("{}", format!("Input must have length = {len}", len=vo::WORD_SIZE));
        }

        number = number - 1;

        self.current_attempt = std::cmp::max(self.current_attempt, number);
        self.attemptions[number].word = value.to_owned();
        
        let mut value_iterator = value.chars();
        let mut target_iterator = self.target.chars();
        for index in 0..vo::WORD_SIZE {
            let input_character = value_iterator.next().unwrap();
            let target_character = target_iterator.next().unwrap();
            let is_input_in_target = self.target.find(input_character);
            match is_input_in_target {
                None => self.attemptions[number].statuses[index] = vo::LetterStatus::NotFound,
                Some(_) => match input_character == target_character {
                    true => self.attemptions[number].statuses[index] = vo::LetterStatus::InCorrectPosition,
                    _ => self.attemptions[number].statuses[index] = vo::LetterStatus::InUncorrectPosition
                }
            }
        }
    }
}