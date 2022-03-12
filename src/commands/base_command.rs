pub trait Command {
    fn execute(&self, arg: Option<String>) -> Result<(), Err>;
}
