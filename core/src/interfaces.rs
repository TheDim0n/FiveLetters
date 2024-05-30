use crate::entities;


pub trait FiveLettersRepo {
    fn close(self);
    fn create_tables(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn fill_tables_with_init_data(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn get_actual_session(&self) -> entities::GameSession;
    fn set_next_solution(&self);
    fn add_attemption(&self, word_id: usize, number: usize, value: String) -> Result<(), Box<dyn std::error::Error>>;
    fn is_word_exists(&self, word: &String) -> bool;
    fn is_word_in_attemptions(&self, word_id: usize, word: &String) -> bool;
}