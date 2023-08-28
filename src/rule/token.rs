/*
   Copyright (c) 2023 lontten
   lec is licensed under Mulan PSL v2.
   You can use this software according to the terms and conditions of the Mulan PSL v2.
   You may obtain a copy of Mulan PSL v2 at:
            http://license.coscl.org.cn/MulanPSL2
   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
   See the Mulan PSL v2 for more details.

 */



#[derive(Debug)]
enum CommandTokenType {
    COMMAND,
    OPTIONS,
    PARAMS,
}

#[derive(Debug)]
pub struct CommandToken {
    //token类型
    typ: CommandTokenType,
    //子命令token
    command: Option<Box<CommandToken>>,
    //选项token列表
    options: Vec<OptToken>,
    //参数列表
    params: Vec<String>,
}

impl CommandToken {
    pub fn new() -> CommandToken {
        CommandToken {
            typ: CommandTokenType::COMMAND,
            command: None,
            options: vec![],
            params: vec![],
        }
    }
}


#[derive(Debug)]
pub struct OptToken {
    //选项
    option: Option<String>,
    //选项参数列表
    params: Vec<String>,
}
