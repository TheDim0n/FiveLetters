use crate::core::value_objects as vo;


pub struct Attempt {
    target: String,
    char_count: usize,
    statuses: Vec<vo::LetterStatus>,
    count: usize
}
impl Attempt {
    pub fn new(world: &str) -> Attempt {
        let char_count = world.chars().count();
        Attempt{
            target: String::from(world.to_lowercase()),
            statuses: vec![vo::LetterStatus::Undefined; char_count],
            count: 0,
            char_count: char_count
        }
    }

    pub fn current(&self) -> &usize { &self.count  }

    pub fn statuses(&self) -> &Vec<vo::LetterStatus> { &self.statuses }

    pub fn solution(&self) -> &String { &self.target }

    pub fn check(&mut self, input: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let input = String::from(input.to_lowercase());

        if self.char_count != input.chars().count() {
            return Err(format!("Input must have length = {len}", len=self.char_count))?
        }
        let mut status = true;
        let mut input_iterator = input.chars();
        let mut target_iterator = self.target.chars();
        for index in 0..self.char_count {
            let input_character = input_iterator.next().unwrap();
            let target_character = target_iterator.next().unwrap();
            let is_input_in_target = self.target.find(input_character);
            match is_input_in_target {
                None => {
                    self.statuses[index] = vo::LetterStatus::NotFound;
                    status = false;
                },
                Some(_) => match input_character == target_character {
                    true => {
                        self.statuses[index] = vo::LetterStatus::InCorrectPosition;
                        status &= true;
                    },
                    _ => {
                        self.statuses[index] = vo::LetterStatus::InUncorrectPosition;
                        status = false;
                    }
                }
            }
        }
        self.count += 1;
        Ok(status)
    }
}