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
    pub fn parse(&self, args: Vec<String>) {
        println!("{:?}", args)
    }
    pub fn execute(&self) {}
    pub fn execute_str(&self) -> &'static str {
        println!("{:#?}", self);
        return "lec";
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