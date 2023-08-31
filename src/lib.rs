// Copyright (c) 2023 lontten
// lec is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

pub use crate::rule::command::*;
use crate::rule::token::{CommandToken, CommandTokenType, OptToken};

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

    pub fn default(&mut self) -> &mut App {
        self.add_option(LecOption::new("version").set_short_name('v'))
    }

    pub fn run(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        self.parse(&args);
        self.execute();
    }

    //添加选项
    pub fn add_option(&mut self, opt: LecOption) -> &mut App {
        self.rule.options.push(opt);
        self
    }

    //添加命令最终执行的函数
    pub fn set_func(&mut self, func: FuncType) -> &mut App {
        self.rule.func = Some(func);
        self
    }

    //添加参数限制列表
    pub fn set_params_limit_list(&mut self, params: Vec<String>) -> &mut App {
        self.rule.params = Some(params);
        self
    }


    //添加子命令
    pub fn add_command(&mut self, command: LecCommand) -> &mut App {
        self.rule.commands.push(command);
        self
    }


    pub fn parse(&self, args: &Vec<String>) {
        let mut t = CommandToken::new();
        t.typ = CommandTokenType::COMMAND;


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
            .add_option(LecOption::new("version").set_short_name('v'))
            .set_func(|opts, args| {
                println!("do func opts:{:?},args:{:?}", opts, args);
                "0.1.0".to_string()
            });


        let c_list = LecCommand::new("list")
            .add_option(
                LecOption::new("all")
                    .set_short_name('a')
            )
            .set_func(|opts: Vec<OptToken>, args: Vec<String>| {
                return format!("list opts:{:?},args:{:?}", opts, args);
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
