pub struct Command {
    pub find: String,
    pub change: String,
    pub exit: bool,

}

impl Command {
    pub fn new(find: String, change: String, exit: bool) -> Command {
        Command { find, change, exit }
    }
}