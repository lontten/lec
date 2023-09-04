// Copyright (c) 2023 lontten
// lec is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.


#[derive(Debug, PartialEq)]
pub enum ArgTokenType {
    OPTIONS,
    CommArgs,
    CommExArgs,
}

#[derive(Debug)]
pub struct OptToken {
    //选项
    pub name: String,
    //选项参数列表
    pub opt_args: Vec<String>,
}

#[derive(Debug)]
pub struct ArgToken {
    //token类型
    pub typ: ArgTokenType,
    //选项token列表
    pub options: Vec<OptToken>,
    //参数列表
    pub args: Vec<String>,
}

#[derive(Debug)]
pub struct CommandToken {
    pub name: String,
    //子命令token
    pub command: Option<Box<CommandToken>>,
    //命令参数列表 或 命令扩展参数列表
    pub args: Vec<ArgToken>,
}

impl CommandToken {
    pub fn new() -> CommandToken {
        CommandToken {
            name: "".to_string(),
            command: None,
            args: vec![],
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{App, AppConfig, ArgLimit, LecCommand, LecOption};

    use super::*;

    #[test]
    fn parse_params_test() {
        let mut app = App::new(AppConfig {
            name: "".to_string(),
            version: "".to_string(),
            author: "".to_string(),
            email: "".to_string(),
        });

        let s1 = vec!["a".to_string(), "b".to_string()];

        app.parse(&s1);
        assert_eq!(app.token.command.is_none(), true);
        assert_eq!(app.token.args.len(), 1);
        assert_eq!(app.token.args[0].typ, ArgTokenType::CommArgs);
        assert_eq!(app.token.args[0].args.len(), 2);
        assert_eq!(app.token.args[0].args[0], "a");
        assert_eq!(app.token.args[0].args[1], "b");
    }

    #[test]
    fn parse_command_test() {
        let mut app = App::new(AppConfig {
            name: "".to_string(),
            version: "".to_string(),
            author: "".to_string(),
            email: "".to_string(),
        });
        app.add_command(LecCommand::new("list"));

        let s1 = vec!["list".to_string()];

        app.parse(&s1);
        assert_eq!(app.token.command.is_none(), false);
        assert_eq!(app.token.args.is_empty(), true);

        let sub_command1 = app.token.command.unwrap();
        assert_eq!(sub_command1.name, "all");
        assert_eq!(sub_command1.command.is_none(), true);
        assert_eq!(sub_command1.args.is_empty(), true);
    }

    #[test]
    fn parse_option_test() {
        let mut app = App::new(AppConfig {
            name: "".to_string(),
            version: "".to_string(),
            author: "".to_string(),
            email: "".to_string(),
        });

        app.set_option_disorder(vec![
            LecOption::new("all")
        ], ArgLimit::None);

        let s1 = vec!["--all".to_string()];

        app.parse(&s1);
        assert_eq!(app.token.command.is_none(), true);
        assert_eq!(app.token.args.len(), 1);

        let arg1 = &app.token.args[0];
        assert_eq!(arg1.typ, ArgTokenType::OPTIONS);
        assert_eq!(arg1.args.len(), 0);

        let opt1 = &arg1.options[0];
        assert_eq!(opt1.name, "all");
        assert_eq!(opt1.opt_args.is_empty(), true);
    }

    #[test]
    fn parse_option_short_test() {
        let mut app = App::new(AppConfig {
            name: "".to_string(),
            version: "".to_string(),
            author: "".to_string(),
            email: "".to_string(),
        });
        app.set_option_disorder(vec![
            LecOption::new("all").set_short_name('a')
        ], ArgLimit::None);

        let s1 = vec!["-a".to_string()];

        app.parse(&s1);
        assert_eq!(app.token.command.is_none(), true);
        assert_eq!(app.token.args.len(), 1);

        let arg1 = &app.token.args[0];
        assert_eq!(arg1.typ, ArgTokenType::OPTIONS);
        assert_eq!(arg1.args.len(), 0);

        let opt1 = &arg1.options[0];
        assert_eq!(opt1.name, "all");
        assert_eq!(opt1.opt_args.is_empty(), true);
    }
}

