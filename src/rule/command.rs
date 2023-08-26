/*
    lec A library for creating powerful modern CLI applications. 一个用于创建功能强大的现代CLI应用程序的库
    Copyright (C) 2023  lontten

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
    pub fn parse(&self, str: &str) {
        println!("{}", str)
    }
    pub fn execute(&self) -> &'static str {
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