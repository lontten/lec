// Copyright (c) 2023 lontten
// lec is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use crate::AppConfig;
use crate::rule::token::OptToken;

#[derive(Debug)]
pub struct LecOption {
    //选项
    name: String,
    //短选项
    short_name: Option<char>,
    //说明
    about: String,
    //参数限制
    opt_arg_limit: ArgLimit,
}

impl LecOption {
    pub fn new(name: &str) -> LecOption {
        LecOption {
            name: name.to_string(),
            short_name: None,
            about: "".to_string(),
            opt_arg_limit: ArgLimit::None,
        }
    }

    //添加选项短命令
    pub fn set_short_name(mut self, name: char) -> LecOption {
        self.short_name = Some(name);
        self
    }
    //添加选项说明
    pub fn set_about(mut self, about: &str) -> LecOption {
        self.about = about.to_string();
        self
    }
    //添加参数限制列表
    pub fn set_arg_limit(mut self, arg_limit: ArgLimit) -> LecOption {
        self.opt_arg_limit = arg_limit;
        self
    }
}


pub type FuncTypeDisOrder = fn(AppConfig, Vec<OptToken>, Vec<String>);
pub type FuncTypeOrder = fn(AppConfig, Vec<OptToken>, Vec<String>, Vec<String>);
pub type FuncTypeExtra = fn(AppConfig, Vec<OptToken>, Vec<OptToken>, Vec<String>, Vec<String>);

#[derive(Debug, PartialEq)]
pub enum OptionTyp {
    // 选项-无序
    Disorder(FuncTypeDisOrder),
    // 选项-有序
    Order(FuncTypeOrder),
    // 选项-扩展（有序）
    Extra(FuncTypeExtra),
    None,
}

#[derive(Debug, PartialEq)]
pub enum ArgLimit {
    //参数数量限制，固定几个
    LimitNum(i32),
    //参数名字限制，固定名字列表
    LimitName(Vec<String>),
    //参数数量不少于
    NoLess(i32),
    //无限制
    None,
}

#[derive(Debug)]
pub struct LecCommand {
    //命令-名字
    pub name: String,

    //子命令列表
    pub commands: Vec<LecCommand>,

    // 选项类型
    pub option_typ: OptionTyp,
    //选项1列表
    pub options1: Vec<LecOption>,
    //选项2列表
    pub options2: Vec<LecOption>,
    //选项参数限制
    pub comm_arg_limit: ArgLimit,
    //扩展选项参数限制
    pub comm_ex_arg_limit: ArgLimit,
}

impl Clone for LecCommand {
    fn clone(&self) -> Self {
        LecCommand {
            name: self.name.clone(),
            commands: vec![],
            option_typ: OptionTyp::None,
            options1: vec![],
            options2: vec![],
            comm_arg_limit: ArgLimit::None,
            comm_ex_arg_limit: ArgLimit::None,
        }
    }
}


impl LecCommand {
    pub fn new(comm: &str) -> LecCommand {
        LecCommand {
            name: comm.to_string(),
            commands: vec![],
            option_typ: OptionTyp::None,
            options1: vec![],
            options2: vec![],
            comm_arg_limit: ArgLimit::None,
            comm_ex_arg_limit: ArgLimit::None,
        }
    }

    //设置选项-无序
    pub fn set_option(mut self, opts: Vec<LecOption>, arg_limit: ArgLimit, func: FuncTypeDisOrder) -> LecCommand {
        self.option_typ = OptionTyp::Disorder(func);
        self.options1 = opts;
        self.comm_arg_limit = arg_limit;
        self
    }

    //设置选项-有序
    pub fn set_option_order(mut self, opts1: Vec<LecOption>,
                            opts2: Vec<LecOption>,
                            arg_limit: ArgLimit,
                            func: FuncTypeOrder) -> LecCommand {
        self.option_typ = OptionTyp::Order(func);
        self.options1 = opts1;
        self.options2 = opts2;
        self.comm_arg_limit = arg_limit;
        self
    }

    //设置选项-扩展（有序）
    pub fn set_option_extra(mut self, opts1: Vec<LecOption>,
                            opts2: Vec<LecOption>,
                            arg_limit: ArgLimit,
                            ex_arg_limit: ArgLimit,
                            func: FuncTypeExtra,
    ) -> LecCommand {
        self.option_typ = OptionTyp::Extra(func);
        self.options1 = opts1;
        self.options2 = opts2;
        self.comm_arg_limit = arg_limit;
        self.comm_ex_arg_limit = ex_arg_limit;
        self
    }
}


#[cfg(test)]
mod tests {
    use crate::{App, AppConfig};

    use super::*;

    #[test]
    fn rule_option_test() {
        let mut app = App::new(AppConfig {
            name: "lec".to_string(),
            version: "0.1.0".to_string(),
            author: "".to_string(),
            email: "".to_string(),
            about: "".to_string(),
        });

        // app.set_option_disorder(vec![
        //     LecOption::new("all")
        // ], ArgLimit::None);

        assert_eq!(app.rule.name, "lec");
        assert_eq!(app.rule.options1.len(), 1);
        assert_eq!(app.rule.commands.len(), 0);
        assert_eq!(app.rule.comm_arg_limit, ArgLimit::None);
        assert_eq!(app.rule.comm_ex_arg_limit, ArgLimit::None);

        let opt1 = &app.rule.options1[0];
        assert_eq!(opt1.name, "all");
        assert_eq!(opt1.short_name, None);
        assert_eq!(opt1.opt_arg_limit, ArgLimit::None);

        // ------------短选项
        app = App::new(AppConfig {
            name: "lec".to_string(),
            version: "0.1.0".to_string(),
            author: "".to_string(),
            email: "".to_string(),
            about: "".to_string(),
        });
        // app.set_option_disorder(vec![
        //     LecOption::new("all").set_short_name('a')
        // ], ArgLimit::None);

        assert_eq!(app.rule.name, "lec");
        assert_eq!(app.rule.options1.len(), 1);
        assert_eq!(app.rule.commands.len(), 0);
        assert_eq!(app.rule.comm_arg_limit, ArgLimit::None);
        assert_eq!(app.rule.comm_ex_arg_limit, ArgLimit::None);

        let opt1 = &app.rule.options1[0];
        assert_eq!(opt1.name, "all");
        assert_eq!(opt1.short_name, Some('a'));
        assert_eq!(opt1.opt_arg_limit, ArgLimit::None);

        // ------------短选项-func
        app = App::new(AppConfig {
            name: "lec".to_string(),
            version: "0.1.0".to_string(),
            author: "".to_string(),
            email: "".to_string(),
            about: "".to_string(),
        });
        // app.set_option_disorder(vec![
        //     LecOption::new("all").set_short_name('a')
        // ], ArgLimit::None);
        // app.set_func(|opts, args, ex_args| {
        //     println!("opts:{:?},args:{:?},ex_args:{:?}", opts, args, ex_args)
        // });

        assert_eq!(app.rule.name, "lec");
        assert_eq!(app.rule.options1.len(), 1);
        assert_eq!(app.rule.commands.len(), 0);
        assert_eq!(app.rule.comm_arg_limit, ArgLimit::None);
        assert_eq!(app.rule.comm_ex_arg_limit, ArgLimit::None);

        let opt1 = &app.rule.options1[0];
        assert_eq!(opt1.name, "all");
        assert_eq!(opt1.short_name, Some('a'));
        assert_eq!(opt1.opt_arg_limit, ArgLimit::None);
    }
}

