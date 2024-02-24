pub trait FiveLettersRepo {
    fn close(self);
    fn create_tables(&self) -> Result<(), Box<dyn std::error::Error>>;
}