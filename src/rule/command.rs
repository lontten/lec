/*
   Copyright (c) 2023 lontten
   lec is licensed under Mulan PSL v2.
   You can use this software according to the terms and conditions of the Mulan PSL v2.
   You may obtain a copy of Mulan PSL v2 at:
            http://license.coscl.org.cn/MulanPSL2
   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
   See the Mulan PSL v2 for more details.

 */

use crate::rule::token::{CommandToken, OptToken};

#[derive(Debug)]
pub struct App {
    store: LecCommand,
    commands: CommandToken,
}

impl App {
    pub fn new() -> App {
        App {
            store: LecCommand::new(""),
            commands: CommandToken::new(),
        }
    }

    //添加选项
    pub fn add_option(mut self, opt: LecOption) -> App {
        self.store.options.push(opt);
        self
    }
    //添加命令最终执行的函数
    pub fn set_func(mut self, func: FuncType) -> App {
        self.func = Some(func);
        self
    }
    //添加子命令
    pub fn add_command(mut self, command: LecCommand) -> App {
        self.store.commands.push(command);
        self
    }
}


#[derive(Debug)]
pub struct LecOption {
    name: String,
    //选项
    short_name: Option<char>,
    //短选项
    params: Vec<String>,    //选项参数列表
}

impl LecOption {
    pub fn new(name: &str) -> LecOption {
        LecOption {
            name: name.to_string(),
            short_name: None,
            params: vec![],
        }
    }

    //添加选项短命令
    pub fn set_short_name(mut self, name: char) -> LecOption {
        self.short_name = Some(name);
        self
    }
}


type FuncType = fn(Vec<OptToken>, Vec<String>) -> String;

#[derive(Debug)]
pub struct LecCommand {
    //子命令列表
    pub commands: Vec<LecCommand>,
    //命令-名字
    pub name: String,
    //选项列表
    pub options: Vec<LecOption>,
    //参数列表
    pub args: Vec<String>,
    pub func: Option<FuncType>,
}

impl LecCommand {
    pub fn new(comm: &str) -> LecCommand {
        LecCommand {
            name: comm.to_string(),
            commands: vec![],
            options: vec![],
            args: vec![],
            func: None,
        }
    }

    //添加选项
    pub fn add_option(&mut self, opt: LecOption) -> &mut LecCommand {
        self.options.push(opt);
        self
    }
    //添加命令最终执行的函数
    pub fn set_func(&mut self, func: FuncType) -> &mut LecCommand {
        self.func = Some(func);
        self
    }


    pub fn parse(&self, args: &Vec<String>) {
        println!("parse:{:?}", args)
    }

    pub fn execute(&self) {}
    pub fn execute_str(&mut self) -> &'static str {
        return "lec";
    }
}
