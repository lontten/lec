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
    config: AppConfig,
    rule: LecCommand,
    token: CommandToken,
}

#[derive(Debug)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub author: String,
    pub email: String,
    pub about: String,
}

impl Clone for AppConfig {
    fn clone(&self) -> Self {
        AppConfig {
            name: self.name.clone(),
            version: self.version.clone(),
            author: self.author.clone(),
            email: self.email.clone(),
            about: self.about.clone(),
        }
    }
}


impl App {
    pub fn new(config: AppConfig) -> App {
        App {
            config,
            rule: LecCommand::new(""),
            token: CommandToken::new(),
        }
    }


    //设置选项-无序
    pub fn set_option(&mut self, opts: Vec<LecOption>, arg_limit: ArgLimit, func: FuncTypeDisOrder) -> &mut App {
        let mut c = self.rule.clone();
        c = c.set_option(opts, arg_limit, func);
        self.rule = c;
        self
    }

    //设置选项-有序
    pub fn set_option_order(&mut self, opts1: Vec<LecOption>, opts2: Vec<LecOption>,
                            arg_limit: ArgLimit, func: FuncTypeOrder) -> &mut App {
        self.rule.option_typ = OptionTyp::Order(func, arg_limit);
        self.rule.options1 = opts1;
        self.rule.options2 = opts2;
        self
    }

    //设置选项-扩展（有序）
    pub fn set_option_extra(&mut self, opts1: Vec<LecOption>,
                            opts2: Vec<LecOption>, arg_limit: ArgLimit,
                            ex_arg_limit: ArgLimit, func: FuncTypeExtra) -> &mut App {
        self.rule.option_typ = OptionTyp::Extra(func, arg_limit, ex_arg_limit);
        self.rule.options1 = opts1;
        self.rule.options2 = opts2;
        self
    }


    pub fn default(&mut self) -> &mut App {
        self.rule.options1 = vec![
            LecOption::new("version").set_short_name('v')
        ];
        self.rule.option_typ = OptionTyp::Disorder(|conf, opts, args| {
            if opts.len() == 0 && args.len() == 0 {
                println!("{:?},version:{}", conf, conf.version);
            }
        }, ArgLimit::None);

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
        let conf = self.config.clone();
        let c = &self.rule;
        parse_command(c, conf, args)
    }

    pub fn execute(&self) {}

    pub fn execute_str(&mut self) -> &'static str {
        return "lec";
    }
}

//匹配子命令，贪婪匹配
pub fn parse_command(c: &LecCommand, conf: AppConfig, args: &Vec<String>) {
    //没有参数
    if args.len() == 0 {
        c.exec(conf, vec![], vec![], vec![], vec![]);
        return;
    }

    let arg = &args[0];
    let c_list = &c.commands;
    for x in c_list {
        if arg.as_str() == x.name {
            parse_command(x, conf.clone(), args)
        }
    }
    parse_opt_or_arg(c, conf.clone(), args)
}

//匹配选项和参数
pub fn parse_opt_or_arg(c: &LecCommand, conf: AppConfig, args: &Vec<String>) {
    let mut index = 0;
    let mut arg = &args[index];
    //显示声明的参数
    let mut arg_list = vec![];
    if arg == "=" {
        loop {
            index += 1;
            arg = &args[index];
            if arg.starts_with("-") || arg.starts_with("--") {
                break;
            }
            arg_list.push(arg.clone());
        }
    }
}

//匹配命令参数
pub fn parse_arg(c: &LecCommand, args: &Vec<String>) {
    let mut index = 0;
    let mut arg = &args[index];
    //显示声明的参数
    let mut arg_list = vec![];
    if arg == "=" {
        loop {
            index += 1;
            arg = &args[index];
            if arg.starts_with("-") || arg.starts_with("--") {
                break;
            }
            arg_list.push(arg.clone());
        }
    }
}

//匹配选项参数
pub fn parse_opt_arg(o: &LecOption, args: &Vec<String>) {
    let mut index = 0;
    let mut arg = &args[index];
    //显示声明的参数
    let mut arg_list = vec![];
    if arg == "=" {
        loop {
            index += 1;
            arg = &args[index];
            if arg.starts_with("-") || arg.starts_with("--") {
                break;
            }
            arg_list.push(arg.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::command::LecOption;

    use super::*;

    #[test]
    fn parse_test() {
        let mut app = App::new(AppConfig {
            name: "lec".to_string(),
            version: "v0.1.0".to_string(),
            author: "".to_string(),
            email: "".to_string(),
            about: "".to_string(),
        });
        app.default()
            .set_option(
                vec![
                    LecOption::new("version").set_short_name('v')
                ],
                ArgLimit::None,
                |conf, opts, args| {
                    println!("version opts:{:?},args:{:?}", opts, args);
                },
            );


        app.set_option(vec![
            LecOption::new("all").set_short_name('a')
        ], ArgLimit::None, |conf, opts, args| {
            println!("list opts:{:?},args:{:?}", opts, args);
        });


        let s1 = vec!["a".to_string(), "b".to_string()];
        app.parse(&s1);


        assert_eq!(app.config.version, "v0.1.0");
        assert_eq!(app.rule.name, "");
        match app.rule.option_typ {
            OptionTyp::Disorder(_, _) => {
                assert!(true);
            }
            OptionTyp::Order(_, _) => {
                assert!(false);
            }
            OptionTyp::Extra(_, _, _) => {
                assert!(false);
            }
            OptionTyp::None => {
                assert!(false);
            }
        }

        // app.func.unwrap()(vec![], s1);

        assert_eq!(app.execute_str(), "lec");
    }

    #[test]
    fn mut_test() {}
}
