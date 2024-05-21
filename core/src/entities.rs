use crate::value_objects as vo;


type Attemptions = [vo::Attemption; vo::ATTEMPT_COUNT];


#[derive(Debug)]
pub struct GameSession {
    id: usize,
    target: String,
    attemptions: Attemptions,
    current_attempt: usize,
    completed: bool
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

// impl From<(&str, &[WordStatuses])> for GameSession{
//     fn from((world, attemptions): (&str, &[WordStatuses])) -> GameSession {
//         let current_attemption = attemptions.len();
//         assert!(current_attemption <= vo::ATTEMPT_COUNT);
//         let mut attemptions_to_save = [[vo::LetterStatus::Undefined; vo::WORD_SIZE]; vo::ATTEMPT_COUNT];
//         for i in 0..current_attemption {
//             attemptions_to_save[i] = attemptions[i];
//         }
//         GameSession{
//             target: String::from(world.to_lowercase()),
//             attemptions: attemptions_to_save,
//             current_attempt: current_attemption
//         }
//     }
// }

impl GameSession {
    pub fn current_attempt(&self) -> &usize { &self.current_attempt  }

    pub fn solution(&self) -> &String { &self.target }

    pub fn attemptions(&self) -> &Attemptions { &self.attemptions }

    pub fn completed(&self) -> bool { self.completed }

    // pub fn check(&mut self, input: &str) -> Result<bool, Box<dyn std::error::Error>> {
    //     let input = String::from(input.to_lowercase());

    //     if vo::WORD_SIZE != input.chars().count() {
    //         return Err(format!("Input must have length = {len}", len=vo::WORD_SIZE))?
    //     }
    //     let mut status = true;
    //     let mut input_iterator = input.chars();
    //     let mut target_iterator = self.target.chars();
    //     for index in 0..vo::WORD_SIZE {
    //         let input_character = input_iterator.next().unwrap();
    //         let target_character = target_iterator.next().unwrap();
    //         let is_input_in_target = self.target.find(input_character);
    //         match is_input_in_target {
    //             None => {
    //                 self.attemptions[self.current_attempt][index] = vo::LetterStatus::NotFound;
    //                 status = false;
    //             },
    //             Some(_) => match input_character == target_character {
    //                 true => {
    //                     self.attemptions[self.current_attempt][index] = vo::LetterStatus::InCorrectPosition;
    //                     status &= true;
    //                 },
    //                 _ => {
    //                     self.attemptions[self.current_attempt][index] = vo::LetterStatus::InUncorrectPosition;
    //                     status = false;
    //                 }
    //             }
    //         }
    //     }
    //     self.current_attempt += 1;
    //     Ok(status)
    // }
}