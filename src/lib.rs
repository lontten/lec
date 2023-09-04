// Copyright (c) 2023 lontten
// lec is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

pub use crate::rule::command::*;
use crate::rule::token::CommandToken;

mod rule;

#[derive(Debug)]
pub struct App {
    version: String,
    author: String,
    email: String,
    rule: LecCommand,
    token: CommandToken,
}

#[derive(Debug)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub author: String,
    pub email: String,
}


impl App {
    pub fn new(config: AppConfig) -> App {
        App {
            version: config.version,
            author: config.author,
            email: config.email,
            rule: LecCommand::new(config.name.as_str()),
            token: CommandToken::new(),
        }
    }


    //设置选项-无序
    pub fn set_option_disorder(&mut self, opts: Vec<LecOption>, arg_limit: ArgLimit) -> &mut App {
        self.rule.option_typ = OptionTyp::OptionDisorder;
        self.rule.options1 = opts;
        self.rule.comm_arg_limit = arg_limit;
        self
    }

    //设置选项-有序
    pub fn set_option_order(&mut self, opts1: Vec<LecOption>, opts2: Vec<LecOption>,
                            arg_limit: ArgLimit) -> &mut App {
        self.rule.option_typ = OptionTyp::OptionOrder;
        self.rule.options1 = opts1;
        self.rule.options2 = opts2;
        self.rule.comm_arg_limit = arg_limit;
        self
    }

    //设置选项-扩展（有序）
    pub fn set_option_extra(&mut self, opts1: Vec<LecOption>,
                            opts2: Vec<LecOption>, arg_limit: ArgLimit,
                            ex_arg_limit: ArgLimit) -> &mut App {
        self.rule.option_typ = OptionTyp::OptionOrder;
        self.rule.options1 = opts1;
        self.rule.options2 = opts2;
        self.rule.comm_arg_limit = arg_limit;
        self.rule.comm_ex_arg_limit = ex_arg_limit;
        self
    }

    //设置命令最终执行的函数
    pub fn set_func(&mut self, func: FuncType) -> &mut App {
        self.rule.func = Some(func);
        self
    }


    pub fn default(&mut self) -> &mut App {
        self.rule.option_typ = OptionTyp::OptionDisorder;
        self.rule.options1 = vec![
            LecOption::new("version").set_short_name('v')
        ];
        self.rule.comm_arg_limit = ArgLimit::None;
        self
    }

    pub fn run(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        self.parse(&args);
        self.execute();
    }

    //添加子命令
    pub fn add_command(&mut self, command: LecCommand) -> &mut App {
        self.rule.commands.push(command);
        self
    }

    pub fn parse(&self, args: &Vec<String>) {
        let mut t = CommandToken::new();


        println!("parse:{:?}", args)
    }

    pub fn execute(&self) {}

    pub fn execute_str(&mut self) -> &'static str {
        return "lec";
    }
}


#[cfg(test)]
mod tests {
    use crate::rule::command::LecOption;

    use super::*;

    #[test]
    fn parse_test() {
        let mut app = App::new(AppConfig {
            name: "".to_string(),
            version: "".to_string(),
            author: "".to_string(),
            email: "".to_string(),
        });
        app.default()
            .set_option_disorder(vec![
                LecOption::new("version").set_short_name('v')
            ], ArgLimit::None)
            .set_func(|opts, args, ex_args| {
                println!("version opts:{:?},args:{:?},ex_args:{:?}", opts, args, ex_args);
            });


        let c_list = LecCommand::new("list")
            .set_option_disorder(vec![
                LecOption::new("all").set_short_name('a')
            ], ArgLimit::None)
            .set_func(|opts, args, ex_args| {
                println!("list opts:{:?},args:{:?},ex_args:{:?}", opts, args, ex_args);
            });

        app.add_command(c_list);


        let s1 = vec!["a".to_string(), "b".to_string()];
        app.parse(&s1);
        // app.func.unwrap()(vec![], s1);

        assert_eq!(app.execute_str(), "lec");
    }

    #[test]
    fn mut_test() {}
}
