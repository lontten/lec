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
    //子命令token
    pub command: Option<Box<CommandToken>>,
    //参数列表
    pub args: Vec<ArgToken>,
}

impl CommandToken {
    pub fn new() -> CommandToken {
        CommandToken {
            command: None,
            args: vec![],
        }
    }
}


#[derive(Debug)]
pub struct OptToken {
    //选项
    pub name: String,
    //选项参数列表
    pub params: Vec<String>,
}
