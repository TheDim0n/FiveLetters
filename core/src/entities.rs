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
            current_attempt: 1,
            completed: false
        }
    }
}

impl GameSession {
    fn get_char_count(&self, value: &String) -> std::collections::HashMap<char, u8> {
        let mut result = std::collections::HashMap::new();

        for char in value.chars() {
            result.entry(char).and_modify(|x| *x += 1).or_insert(1);
        }
        result
    }

    pub fn add_attemption(&mut self, value: &String, mut number: usize) {
        if vo::WORD_SIZE != value.chars().count() {
            panic!("{}", format!("Input must have length = {len}", len=vo::WORD_SIZE));
        }

        self.current_attempt = std::cmp::max(self.current_attempt, number + 1);
        if self.current_attempt == vo::ATTEMPT_COUNT + 1 { self.completed = true };

        number = number - 1;
        let value = &value.to_lowercase();
        self.attemptions[number].word = value.to_owned();

        let target_char_count = self.get_char_count(&self.target);
        let mut value_char_count = self.get_char_count(value);

        let initial_target_char_count = target_char_count.clone();
        
        let mut value_iterator = value.chars();
        let mut target_iterator = self.target.chars();
        for index in 0..vo::WORD_SIZE {
            let input_character = value_iterator.next().unwrap();
            let target_character = target_iterator.next().unwrap();
            let input_char_count = match target_char_count.get(&input_character) {
                Some(count) => count,
                None => &0
            };
            let initial_input_char_count = match initial_target_char_count.get(&input_character) {
                Some(count) => count,
                None => &0
            };
            let value_input_char_count = match value_char_count.get(&input_character) {
                Some(count) => count,
                None => &0
            };
            match *input_char_count {
                0 => self.attemptions[number].statuses[index] = vo::LetterStatus::NotFound,
                _ => {
                    match input_character == target_character {
                        true => self.attemptions[number].statuses[index] = vo::LetterStatus::InCorrectPosition,
                        _ => match value_input_char_count.cmp(initial_input_char_count) {
                            std::cmp::Ordering::Equal => self.attemptions[number].statuses[index] = vo::LetterStatus::InUncorrectPosition,
                            std::cmp::Ordering::Greater => {
                                self.attemptions[number].statuses[index] = vo::LetterStatus::NotFound;
                                value_char_count.entry(input_character).and_modify(|x| *x -= 1);
                            }
                            std::cmp::Ordering::Less => self.attemptions[number].statuses[index] = vo::LetterStatus::InUncorrectPosition,
                        }
                    }
                } 
            }
        }
        self.completed = value == &self.target;
    }
}