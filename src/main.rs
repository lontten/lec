/*
   Copyright (c) 2023 lontten
   lec is licensed under Mulan PSL v2.
   You can use this software according to the terms and conditions of the Mulan PSL v2.
   You may obtain a copy of Mulan PSL v2 at:
            http://license.coscl.org.cn/MulanPSL2
   THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
   See the Mulan PSL v2 for more details.

 */

use crate::rule::command::Command;

mod rule;

fn main() {
    let c = Command::new();

    let args: Vec<String> = std::env::args().collect();
    c.parse(args);
    c.execute();
}



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

        assert_eq!(c.execute_str(), "lec");


        //一个参数
        let c = Command::new();

        let s1 = vec!["a".to_string()];
        c.parse(s1);

        assert_eq!(c.execute_str(), "lec a");


        //多个参数
        let c = Command::new();
        let s1 = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        c.parse(s1);

        assert_eq!(c.execute_str(), "lec a b c");
    }
}
