// Copyright (c) 2023 lontten
// lec is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.


#[derive(Debug, PartialEq)]
pub enum CommandTokenType {
    OPTIONS,
    PARAMS,
}

#[derive(Debug)]
pub struct OptToken {
    //选项
    pub name: String,
    //选项参数列表
    pub params: Vec<String>,
}

#[derive(Debug)]
pub struct ArgToken {
    //token类型
    pub typ: CommandTokenType,
    //选项token列表
    pub options: Vec<OptToken>,
    //参数列表
    pub params: Vec<String>,
}

#[derive(Debug)]
pub struct CommandToken {
    pub name: String,
    //子命令token
    pub command: Option<Box<CommandToken>>,
    //参数列表
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
    use crate::{App, AppConfig};

    use super::*;

    #[test]
    fn parse_test() {
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
        assert_eq!(app.token.args[0].typ, CommandTokenType::PARAMS);
        assert_eq!(app.token.args[0].params.len(), 2);
        assert_eq!(app.token.args[0].params[0], "a");
        assert_eq!(app.token.args[0].params[1], "b");
    }

    #[test]
    fn mut_test() {}
}

