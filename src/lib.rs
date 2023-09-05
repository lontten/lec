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
        self.rule.option_typ = OptionTyp::Order(func);
        self.rule.options1 = opts1;
        self.rule.options2 = opts2;
        self.rule.comm_arg_limit = arg_limit;
        self
    }

    //设置选项-扩展（有序）
    pub fn set_option_extra(&mut self, opts1: Vec<LecOption>,
                            opts2: Vec<LecOption>, arg_limit: ArgLimit,
                            ex_arg_limit: ArgLimit, func: FuncTypeExtra) -> &mut App {
        self.rule.option_typ = OptionTyp::Extra(func);
        self.rule.options1 = opts1;
        self.rule.options2 = opts2;
        self.rule.comm_arg_limit = arg_limit;
        self.rule.comm_ex_arg_limit = ex_arg_limit;
        self
    }


    pub fn default(&mut self) -> &mut App {
        self.rule.options1 = vec![
            LecOption::new("version").set_short_name('v')
        ];
        self.rule.comm_arg_limit = ArgLimit::None;


        self.rule.option_typ = OptionTyp::Disorder(|conf, opts, args| {
            if opts.len() == 0 && args.len() == 0 {
                println!("{:?},version:{}", conf, conf.version);
            }
        });

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
        let mut t = CommandToken::new();
        match self.rule.option_typ {
            OptionTyp::Disorder(func) => {
                func(conf, vec![], vec![])
            }
            OptionTyp::Order(func) => {
                func(conf, vec![], vec![], vec![])
            }
            OptionTyp::Extra(func) => {
                func(conf, vec![], vec![], vec![], vec![])
            }
            OptionTyp::None => {
                println!("no set option type");
            }
        }
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


        assert_eq!(app.version, "v0.1.0");
        assert_eq!(app.rule.name, "lec");
        match app.rule.option_typ {
            OptionTyp::Disorder(_) => {
                assert!(true);
            }
            OptionTyp::Order(_) => {
                assert!(false);
            }
            OptionTyp::Extra(_) => {
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
