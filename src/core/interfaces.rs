use crate::core::entities;


pub trait FiveLettersRepo {
    fn close(self);
    fn create_tables(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn fill_tables_with_init_data(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn get_actual_session(&self) -> entities::GameSession;
    fn set_next_solution(&self);
}