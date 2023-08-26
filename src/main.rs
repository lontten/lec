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

use crate::rule::command::Command;

mod rule;

fn main() {
    let c = Command::new();

    for x in std::env::args() {
        println!("{}", x);
        c.parse(x.as_str());
    }
    c.execute();
}


fn parse(args: Vec<&str>) {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {

        //没有任何参数、选项
        let c = Command::new();
        let s1 = vec![];
        for x in s1 {
            c.parse(x);
        }

        assert_eq!(c.execute(), "lec");


        //一个参数
        let c = Command::new();
        
        let s1 = vec!["a"];
        for x in s1 {
            c.parse(x);
        }

        assert_eq!(c.execute(), "lec a");


        //多个参数
        let c = Command::new();
        let s1 = vec!["a", "b", "c"];
        for x in s1 {
            c.parse(x);
        }

        assert_eq!(c.execute(), "lec a b c");
    }
}
