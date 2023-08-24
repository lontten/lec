#[derive(Debug)]
pub struct Command {
    //子命令列表
    commands: Vec<Command>,
    //选项列表
    options: Vec<Option>,
    //参数列表
    params: Vec<String>,
}

impl Command {
    pub fn new() -> Command {
        Command {
            commands: vec![],
            options: vec![],
            params: vec![],
        }
    }
    pub fn get_signal(&self) -> &Command {
        self
    }
    pub fn parse(&self, str: String) {
        println!("{}", str)
    }
    pub fn execute(&self) {
        println!("{:#?}", self)
    }
}

#[derive(Debug)]
pub struct Option {
    option: String,
    //选项
    short_option: char,
    //短选项
    params: Vec<String>,    //选项参数列表
}