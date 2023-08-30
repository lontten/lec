/*
   Copyright (c) 2023 lontten
   lec is licensed under Mulan PSL v2.
   You can use this software according to the terms and conditions of the Mulan PSL v2.
   You may obtain a copy of Mulan PSL v2 at:
            http://license.coscl.org.cn/MulanPSL2
   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
   See the Mulan PSL v2 for more details.

 */

use crate::rule::token::OptToken;

#[derive(Debug)]
pub struct LecOption {
    //选项
    name: String,
    //短选项
    short_name: Option<char>,
    //说明
    title: String,
    //选项参数列表限制，None表示不限制，Some时，用户输入的参数必须在列表中；为空数组表示不能有参数
    params: Option<Vec<String>>,
}

impl LecOption {
    pub fn new(name: &str) -> LecOption {
        LecOption {
            name: name.to_string(),
            short_name: None,
            title: "".to_string(),
            params: None,
        }
    }

    //添加选项短命令
    pub fn set_short_name(mut self, name: char) -> LecOption {
        self.short_name = Some(name);
        self
    }
    //添加选项说明
    pub fn set_title(mut self, title: &str) -> LecOption {
        self.title = title.to_string();
        self
    }
    //添加参数限制列表
    pub fn set_params_limit_list(mut self, params: Vec<String>) -> LecOption {
        self.params = Some(params);
        self
    }
}


pub type FuncType = fn(Vec<OptToken>, Vec<String>) -> String;

#[derive(Debug)]
pub struct LecCommand {
    //子命令列表
    pub commands: Vec<LecCommand>,
    //命令-名字
    pub name: String,
    //选项列表
    pub options: Vec<LecOption>,
    //选项参数列表限制，None表示不限制，Some时，用户输入的参数必须在列表中；为空数组表示不能有参数
    pub params: Option<Vec<String>>,
    pub func: Option<FuncType>,
}

impl LecCommand {
    pub fn new(comm: &str) -> LecCommand {
        LecCommand {
            name: comm.to_string(),
            commands: vec![],
            options: vec![],
            params: None,
            func: None,
        }
    }

    //添加选项
    pub fn add_option(mut self, opt: LecOption) -> LecCommand {
        self.options.push(opt);
        self
    }
    //添加参数限制列表
    pub fn set_params_limit_list(mut self, params: Vec<String>) -> LecCommand {
        self.params = Some(params);
        self
    }
    //添加命令最终执行的函数
    pub fn set_func(mut self, func: FuncType) -> LecCommand {
        self.func = Some(func);
        self
    }
}
