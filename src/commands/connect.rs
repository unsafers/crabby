mod base_command;

struct Connect {
}

impl base_command::Command for Connect {
    fn execute(&self, arg: Option<String>) -> Result<(), Err> {
        Ok()
    }
}
