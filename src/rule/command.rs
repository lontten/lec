/*
   Copyright (c) 2023 lontten
   lec is licensed under Mulan PSL v2.
   You can use this software according to the terms and conditions of the Mulan PSL v2.
   You may obtain a copy of Mulan PSL v2 at:
            http://license.coscl.org.cn/MulanPSL2
   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
   See the Mulan PSL v2 for more details.

 */

use crate::print_info;
use crate::rule::token::{CommandToken, OptToken};

#[derive(Debug)]
pub struct Opt {
    option: String,
    //选项
    short_option: char,
    //短选项
    params: Vec<String>,    //选项参数列表
}

#[derive(Debug)]
pub struct Command<F>
    where F: Fn(Vec<OptToken>, Vec<String>) -> String
{
    //子命令列表
    pub commands: Vec<Command<F>>,
    //选项列表
    pub options: Vec<Opt>,
    //参数列表
    pub args: Vec<String>,
    pub func: Option<F>,
}


#[derive(Debug)]
pub struct App<F>
    where F: Fn(Vec<OptToken>, Vec<String>) -> String
{
    store: Command<F>,
    commands: CommandToken,
}


impl<F> Command<F>
    where F: Fn(Vec<OptToken>, Vec<String>) -> String
{
    pub fn new() -> Command<F> {
        Command {
            commands: vec![],
            options: vec![],
            args: vec![],
            func: None,
        }
    }
    pub fn parse(&self, args: &Vec<String>) {
        println!("{:?}", args)
    }
    pub fn execute(&self) {}
    pub fn execute_str(&mut self) -> &'static str {

        return "lec";
    }
}
