pub trait FiveLettersRepo {
    fn close(self);
    fn create_tables(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn fill_tables_with_init_data(&self) -> Result<(), Box<dyn std::error::Error>>;
}